mod option;

use std::{collections::HashMap, fs::{metadata, read_dir, self}, io, path::Path};
use crate::helpers::{file::get_extension, threadpool::ThreadPool, movie};
use colored::Colorize;
use self::option::TranscodeOption;
use super::{get_args_parameter, Runnable, OPTION_SEPARATOR};

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
    file_path: String,
    /// Command options
    cmd_options: HashMap<String, String>,
}

impl Runnable for Transcode {
    fn run(&self) -> Result<()> {
        let mut transcode_option = TranscodeOption::new();

        // --help
        if self.cmd_options.contains_key("h") || self.cmd_options.contains_key("help") {
            println!("\n{}\n", usage());
            return Ok(());
        }

        for (option, value) in &self.cmd_options {
            match option.as_str() {
                "d" => transcode_option.set_delete(),
                "t" | "thread" => transcode_option.set_thread(value)?,
                "e" | "extensions" => transcode_option.extensions_from(value)?,
                arg => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput, 
                        format!("\nUnkown argument {}\n", arg)
                    ));
                },
            };
        }

        let thread_pool = ThreadPool::new(transcode_option.thread);

        match metadata(&self.file_path) {
            Ok(md) if md.is_file() => {
                transcode_file(&self.file_path, &transcode_option, &thread_pool)
            },
            Ok(md) if md.is_dir() => {
                transcode_dir(&self.file_path, &transcode_option, &thread_pool)
            },
            Ok(_) => return Err(io::Error::new(
                io::ErrorKind::InvalidInput, 
                format!("\n{}\nsearch error: unknown file\n\n", self.file_path)
            )),
            Err(err) => {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound, 
                    format!("\n{}\nread error: {}\n\n", self.file_path, err)
                ));
            }            
        }

        Ok(())
    }
}

fn transcode_file(file_path: &String, transcode_option: &TranscodeOption, thread_pool: &ThreadPool) {
    let extension = get_extension(file_path).to_lowercase();
    
    if !transcode_option.has_extension(&extension) {
        return ();
    }

    let file_path = file_path.clone();
    let delete_after = transcode_option.delete;
    thread_pool.execute(move || {
        println!("Transcoding start {file_path}");
        match movie::to_mp4(&file_path, None) {
            Ok(_) if delete_after => match fs::remove_file(&file_path) {
                Err(err) => {
                    println!("{}", err.to_string().on_red());
                },
                _ => (),
            },
            Err(err) => {
                println!("{}", err.to_string().on_red())
            },
            _ => (),
        }
    });
}

fn transcode_dir(dir_path: &String, transcode_option: &TranscodeOption, thread_pool: &ThreadPool) {
    for entry in read_dir(Path::new(&dir_path)).unwrap() {
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
    Transcode video files. Need ffmpeg installed
    --help
    -d  Delete original file after transcoding
    -e <string> --extensions=<string>    only transcode files with these extensions, separated by '{OPTION_SEPARATOR}'
    -t <int> --thread=<int>    Number of max thread used
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
