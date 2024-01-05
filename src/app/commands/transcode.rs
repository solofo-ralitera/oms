mod option;

use std::{collections::HashMap, fs, io, path::Path, sync::{Arc, Mutex}};
use crate::helpers::{file::{get_extension, self}, threadpool::ThreadPool, video};
use colored::Colorize;
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
        println!("Transcode running");
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
            Ok(md) if md.is_dir() => {
                transcode_dir(&self.file_path, &transcode_option, &thread_pool)
            },
            Ok(_) => {
                *b_isrunning = false;
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("\n{}\nsearch error: unknown file\n\n", self.file_path)
                ));
            },
            Err(err) => {
                *b_isrunning = false;
                return Err(io::Error::new(
                    io::ErrorKind::NotFound, 
                    format!("\n{}\nread error: {}\n\n", self.file_path, err)
                ));
            }            
        }
        *b_isrunning = false;
        Ok(())
    }
}

fn transcode_file(file_path: &String, transcode_option: &TranscodeOption, thread_pool: &ThreadPool) {
    let extension = get_extension(file_path).to_lowercase();

    if !file::VIDEO_EXTENSIONS.contains(&extension.as_str()) {
        return ();
    }

    if !transcode_option.has_extension(&extension) {
        return ();
    }

    let file_path = file_path.clone();
    let delete_after = transcode_option.delete;
    let output_format = transcode_option.output_format.clone();
    thread_pool.execute(move || {
        if extension.eq(&output_format) {
            return;
        }
        match video::transcode(&file_path, None, &output_format) {
            Ok(dest_output) if dest_output.is_some() && delete_after => match fs::remove_file(&file_path) {
                Err(err) => {
                    println!("{}", err.to_string().on_red());
                },
                _ => (),
            },
            Ok(dest_output) if dest_output.is_none() => {
                println!("{} already exists", file_path.blue());
            },
            Err(err) => {
                println!("{}", err.to_string().on_red())
            },
            _ => (),
        }
    });
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
    Transcode video files.
    Prerequists:
        Need to install ffmpeg

    --help
    -d  Delete original file after transcoding
    -e <string> --extensions=<string>    only transcode files with these extensions, separated by '{OPTION_SEPARATOR}'
    -t <int> --thread=<int>    Number of max thread used
    -o <string> --output=<string>   Output extension
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
