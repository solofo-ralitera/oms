mod option;

use std::{collections::HashMap, fs, io, path::Path, sync::{Arc, Mutex}, thread};
use crate::helpers::{file, media::video, threadpool::ThreadPool};
use colored::Colorize;
use regex::Regex;
use self::option::TranscodeOption;
use super::{get_args_parameter, Runnable, OPTION_SEPARATOR};
use once_cell::sync::Lazy;

type Result<T> = std::result::Result<T, std::io::Error>;

/// # transcode command
/// 
/// Transcode video files for streaming purpose.
/// Need ffmpeg installed
/// 
/// ## Usage
/// `cargo run -- transcode --extensions="avi,webm" /home/solofo/Videos/Webcam`
/// 
pub struct Transcode {
    /// path of the file/dir to transcode
    pub file_path: String,
    /// Command options
    pub cmd_options: HashMap<String, String>,
}

static TRANSCODE_RUNNING: Lazy<Arc<Mutex<bool>>> = Lazy::new(|| {
    return Arc::new(Mutex::new(false));
});

impl Runnable for Transcode {
    fn run(&self) -> Result<()> {
        let b_isrunning = Arc::clone(&TRANSCODE_RUNNING);
        let mut b_isrunning = b_isrunning.lock().unwrap();

        if *b_isrunning == true {
            return Err(io::Error::new(
                io::ErrorKind::AddrInUse, 
                format!("\nTranscode is already running\n")
            ));
        }
        *b_isrunning = true;
        
        let mut transcode_option = TranscodeOption::new();

        // --help
        if self.cmd_options.contains_key("h") || self.cmd_options.contains_key("help") {
            *b_isrunning = false;
            println!("\n{}\n", usage());
            return Ok(());
        }

        for (option, value) in &self.cmd_options {
            match option.as_str() {
                "d" => transcode_option.set_delete(),
                "t" | "thread" => transcode_option.set_thread(value)?,
                "e" | "extensions" => transcode_option.extensions_from(value)?,
                "o" | "output" => transcode_option.set_output(value)?,
                "s" | "split" => transcode_option.set_split(value)?,
                arg => {
                    *b_isrunning = false;
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput, 
                        format!("\nUnkown argument {}\n", arg)
                    ));
                },
            };
        }

        let thread_pool = ThreadPool::new(transcode_option.thread);

        match fs::metadata(&self.file_path) {
            Ok(md) if md.is_file() => {
                transcode_file(&self.file_path, &transcode_option, &thread_pool)
            },
            Ok(md) if md.is_dir() && transcode_option.split == 0 => {
                transcode_dir(&self.file_path, &transcode_option, &thread_pool)
            },
            Ok(_) if transcode_option.split > 0 => {
                *b_isrunning = false;
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("\n{}\ntranscode error: split only available for file, not directory\n\n", self.file_path)
                ));
            },
            Ok(_) => {
                *b_isrunning = false;
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("\n{}\ntranscode error: unknown file\n\n", self.file_path)
                ));
            },
            Err(err) => {
                *b_isrunning = false;
                return Err(io::Error::new(
                    io::ErrorKind::NotFound, 
                    format!("\n{}\ntranscode error: {}\n\n", self.file_path, err)
                ));
            }            
        }
        *b_isrunning = false;
        Ok(())
    }
}


fn transcode_file(file_path: &String, transcode_option: &TranscodeOption, thread_pool: &ThreadPool) {
    if transcode_option.split > 0 {
        transcode_file_split(file_path, transcode_option);
    } else {
        transcode_file_single(file_path, transcode_option, thread_pool);
    }
}

fn transcode_file_single(file_path: &String, transcode_option: &TranscodeOption, thread_pool: &ThreadPool) {
    let extension = file::get_extension(file_path).to_lowercase();
    if !file::is_video_file(file_path) {
        return ();
    }
    if !transcode_option.has_extension(&extension) {
        return ();
    }
    let file_path = file_path.clone();
    let delete_after = transcode_option.delete;
    let output_format = transcode_option.get_output(&extension);

    thread_pool.execute(move || {
        if !video::need_reencode(&file_path) {
            return;
        }
        match video::transcode(&file_path, None, &output_format) {
            Ok(dest_output) if dest_output.is_some() && delete_after => match fs::remove_file(&file_path) {
                Ok(_) => {
                    let dest_output = dest_output.unwrap_or_default();
                    if dest_output.eq(&format!("{file_path}.{output_format}")) {
                        // Rename output if same extension but need to re-encode .mp4.mp4
                        if let Err(err) = file::rename_file(&dest_output, &file_path) {
                            println!("{}{}", "Transcode error: unable to rename output file, ".red(), err.to_string().red())
                        }
                    }
                },
                Err(err) => println!("{}{}", "Transcode error: unable to delete original file, ".red(), err.to_string().red()),
            },
            Ok(dest_output) if dest_output.is_none() => {
                println!("{}{}", "Transcode warn: Output already exists ".blue(), file_path.blue());
            },
            Err(err) => {
                println!("{}{}", "Transcode error: ".red(), err.to_string().red())
            },
            _ => (),
        }
    });
}

/// 
/// This method can resume encoding
/// - split into multiple parts
/// - Encode theses parts
/// - Concat them when finished
/// 
fn transcode_file_split(file_path: &String, transcode_option: &TranscodeOption) {
    if !file::is_video_file(file_path) {
        return;
    }
    // Don't re-encode part-
    let re_part = Regex::new(r"^part\-[0-9]{7}\.").unwrap();
    if re_part.is_match(&file::get_file_name(file_path)) {
        return;
    }

    let directory = file::get_file_dir(file_path);
    if directory.is_none() {
        println!("{}{}", "Transcode error: unable to find directory for ".red(), file_path.red());
        return;
    }
    let directory = directory.unwrap();
    let directory = Path::new(&directory);
    let directory = directory.join(file::remove_extension(&file::get_file_name(file_path)));
    
    if directory.is_file() {
        println!("{}{}", "Transcode error: file exists instead of directory: ".red(), directory.display().to_string().red());
        return;
    }
    
    if ! directory.exists() {
        let _ = fs::create_dir(&directory);
        video::split_video(file_path, &directory.display().to_string(), transcode_option.split);
    }

    let transcode_option_t = transcode_option.clone();
    let file_path = file_path.to_string();
    let directory_t = directory.clone();
    match thread::spawn(move || {
        let thread_pool = ThreadPool::new(transcode_option_t.thread);
        let file_names: Vec<String> = file::scan_files(&directory_t)
            .into_iter()
            .filter(|f| re_part.is_match(&file::get_file_name(f)))
            .collect();

        for file in file_names {
            if file.contains(".oms_transcoded.") || file.contains(".oms_transcode_temp.") {
                continue;
            }
            let transcode_option = transcode_option_t.clone();
            thread_pool.execute(move || {
                let extension = file::get_extension(&file).to_lowercase();
                let output_format = transcode_option.get_output(&extension);
                
                let dest_path = format!("{}.oms_transcode_temp.{}", file, video::transcode_extension(&output_format));
                let _ = fs::remove_file(&dest_path);

                match video::transcode(&file, Some(&dest_path), &output_format) {
                    Ok(dest_output) if dest_output.is_some() => match fs::remove_file(&file) {
                        Err(err) => {
                            panic!("{}{}: {}", "Transcode error: unable to delete split file ".red(), file.red(), err.to_string().red())
                        },
                        _ => {
                            let dest_output = dest_output.unwrap();
                            let _ = fs::rename(&dest_output, &dest_output.replace(".oms_transcode_temp.", ".oms_transcoded."));
                            println!("Transcode end for {file}");
                        },
                    },
                    Err(err) => panic!("{}{}", "Transcode error: ".red(), err.to_string().red()),
                    _ => (),
                };
            });
        }
    }).join() {
        Ok(_) => {
            // Create filelist for ffmpeg concat
            let output = directory.join("oms_output.txt");
            let mut file_names: Vec<String> = file::scan_files(&directory)
                .into_iter()
                .filter(|f| f.contains(".oms_transcoded."))
                .map(|f| format!("file '{}'", file::get_file_name(&f)))
                .collect();
            file_names.sort();
            let _ = file::write_file_content(
                &output,
                &file_names.join("\n"),
                false
            );
            // Output file
            let extension = file::get_extension(&file_path).to_lowercase();
            let output_format = transcode_option.get_output(&extension);
            let output_file = format!("{}.{output_format}", directory.display().to_string());
            // Concat all part
            match video::concat_video(&output.display().to_string(), &output_file) {
                Err(err) => {
                    println!("{} {}", "Transcode finished with error on concat:".red(), err.to_string().red());
                },
                Ok(r) if r.is_none() => {
                    println!("{} {} {}", "Transcode finished with warning:".yellow(), output_file.yellow(), "already exists".yellow());
                },
                Ok(output) => {
                    // Remove split directory
                    file::scan_files(&directory)
                        .into_iter()
                        .for_each(|f| {
                            if file::get_file_name(&f).eq("oms_output.txt") || f.contains(".oms_transcoded.") || f.contains(".oms_transcode_temp.") {
                                let _ = fs::remove_file(&f);
                            }
                        });
                    // Remove directory
                    if let Err(err) = fs::remove_dir(&directory) {
                        println!("{} {} {}", "Enable to remove tmp directory".yellow(), directory.display().to_string().yellow(), err.to_string().yellow());
                    }
                    // remove source file if -d
                    if transcode_option.delete {
                        if let Err(err) =  fs::remove_file(&file_path) {
                            println!("{} {} {}", "Enable to remove original file".yellow(), file_path.to_string().yellow(), err.to_string().yellow());
                        }
                    }
                    // Rename output
                    let mut output = output.unwrap();
                    let extension = file::get_extension(&output);
                    if output.ends_with(&format!(".{}.{}", extension, extension)) {
                        if let Ok(o) = file::rename_file(&output, &file::remove_extension(&output)) {
                            output = o;
                        }
                    }

                    println!("Transcode finished on {} -> {}", file_path, output.blue());
                },
            };
        },
        Err(err) => {
            println!("{} {}: {}", "Transcode finished with error on".red(), file_path.red(), err.downcast_ref::<&str>().unwrap_or(&""));
        },
    }
}

fn transcode_dir(dir_path: &String, transcode_option: &TranscodeOption, thread_pool: &ThreadPool) {
    for entry in fs::read_dir(Path::new(&dir_path)).unwrap() {
        let path = entry.unwrap().path();
        if path.is_file() {
            transcode_file(&path.to_str().unwrap().to_string(), transcode_option, thread_pool)
        } else if path.is_dir() {
            transcode_dir(&path.to_str().unwrap().to_string(), transcode_option, thread_pool)
        }
    }
}

/// Help message for this command
pub fn usage() -> String {
    format!("\
transcode [options] <file_path|directory_path>
    Transcode video files into a streamable format.
    Transcode only files that are not encoded in VP8, VP9, H.264, AV1, Ogg
    Prerequists:
        Need to install ffmpeg

    --help
    -d  Delete original file after transcoding
    -e <string> --extensions=<string>   Only transcode files with these extensions, separated by '{OPTION_SEPARATOR}'
    -t <int> --thread=<int> Number of threads used
    -o <string> --output=<string>   Output extension, default mp4, (Output can be something like flv>webm,avi>mp4,mp4)
    -s <int> --split=<int>  Number in second. Split the source file into x seconds, then transcode these parts. 
                            Use this if you want to resume transcoding. Available only for transcoding single file
")
}

pub fn build_cmd(args: &Vec<String>, options: HashMap<String, String>) -> Result<Transcode> {
    let file_path = get_args_parameter(
        args,
        args.len() - 1, // Get last agruments
        "\nread error: 'file_path' parameter required\n"
    ).unwrap_or_default();
    
    Ok(Transcode {
        file_path: file_path.to_string(),
        cmd_options: options,
    })
}
