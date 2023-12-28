mod pdf;
mod movie;
mod option;

use std::{io::{self, Error, ErrorKind}, collections::HashMap, fs::{metadata, read_dir}, thread, path::Path, sync::mpsc::{self, Sender}};
use crate::helpers::file::{get_extension, get_file_name};
use super::{Runnable, get_args_parameter};
use self::{pdf::PdfInfo, movie::MovieInfo, option::InfoOption};


/// # Info command
/// 
/// Finds information related to the file
/// 
/// ## Usage
///
/// `oms info /home/me/movie.mp4`
/// 
/// cargo run -- info --elastic-dsn="http://localhost:9200" --cache-path="/media/solofo/MEDIA/.oms" "/media/solofo/MEDIA/films/"
/// 
/// 
/// ## Features
/// 
/// * [ ] File information: TODO
/// * [ ] Movie information: TODO
/// * [ ] Read office file: TODO
/// 
pub struct Info {
    /// the path of the file
    pub file_path: String,
    /// Command options
    pub cmd_options: HashMap<String, String>,
}

static mut INFO_RUNNING: bool = false;

impl Runnable for Info {
      /// Start processing the command
     fn run(&self) -> Result<(), std::io::Error> {
        unsafe {
            if INFO_RUNNING == true {
                return Err(Error::new(
                    ErrorKind::AddrInUse, 
                    format!("\nInfo is already running\n")
                ));
            }
            INFO_RUNNING = true;
        }
        let (tx, rx) = mpsc::channel();
        let mut info_option = InfoOption::new();
        let mut file_path = self.file_path.to_string();
        // --help
        if self.cmd_options.contains_key("h") || self.cmd_options.contains_key("help") {
            print_usage();
            unsafe {
                INFO_RUNNING = false;
            }
            return Ok(());
        }

        info_option.set_basepath(&file_path)?;
        for (option, value) in &self.cmd_options {
            match option.as_str() {
                "hide-preview" => info_option.hide_preview(),
                "elastic-dsn" => info_option.set_elastic(value)?,
                "list" => { 
                    info_option.set_list(value)?; // Files are provided in option
                    file_path.clear(); // Ignore the file in last option
                },
                arg => return Err(Error::new(
                    ErrorKind::InvalidInput, 
                    format!("\nUnkown argument {}\n", arg)
                )),
            };
        }
        
        let display_result = info_option.display_preview;

        // Juste one thread to limit api call
        thread::spawn(move || {
            // if files are provided in option as list
            if info_option.list.len() > 0 {
                file_info_from_list(&info_option, tx);
                return;
            }

            match metadata(&file_path) {
                Ok(md) if md.is_dir() => {
                    dir_info(&file_path, &info_option, tx.clone());
                },
                Ok(md) if md.is_file() => {
                    file_info(&file_path, &info_option, tx.clone());
                },
                _ => file_info(&file_path, &info_option, tx.clone()),
            };
        });
        
        for message in rx {
            if display_result == true {
                println!("{message}");
            }
        }

        unsafe {
            INFO_RUNNING = false;
        }
        Ok(())
    }
}

fn dir_info(dir_path: &String, info_option: &InfoOption, tx: Sender<String>) {
    for entry in read_dir(Path::new(&dir_path)).unwrap() {
        let path = entry.unwrap().path();
        if path.is_file() {
            file_info(&path.to_str().unwrap().to_string(), &info_option, tx.clone())
        } else if path.is_dir() {
            dir_info(&path.to_str().unwrap().to_string(), &info_option, tx.clone())
        }
    }
}

fn file_info(file_path: &String, info_option: &InfoOption, tx: Sender<String>)  {
    let extension = get_extension(&file_path).to_lowercase();
    match extension.as_str() {
        "pdf" => PdfInfo { file_path: &file_path}.info(tx),
        "" | "mp4" | "mkv" | "avi" | "flv" | "mpg" | "mpeg" | "divx" => MovieInfo { 
            movie_raw_name: &get_file_name(&file_path),
            file_path: &file_path,
            info_option: &info_option,
        }.info(tx),
        "db" | "srt" | "nfo" | "idx" | "sub" | "bup" | "ifo" | "vob" | "sfv" => (),
        _ => print!("{file_path}: Format not supported"),
    };
}

fn file_info_from_list(info_option: &InfoOption, tx: Sender<String>) {
    for file_path in &info_option.list {
        match metadata(&file_path) {
            Ok(md) if md.is_dir() => {
                dir_info(file_path, info_option, tx.clone());
            },
            Ok(md) if md.is_file() => {
                file_info(file_path, info_option, tx.clone());
            },
            _ => (),
        };
    }
}

/// Help message for this command
pub fn usage() -> &'static str {
    "\
info [file_path/dir_path]
    Display/get file informations
    --help
    --cache-path=<string>   Cache path, default ./.oms/
    --elastic-dsn=<string>  Elastic search server
    --hide-preview=<bool>   Mute display
    --list=<sting>          Path of a file containing the list of files to parse

    For movies: info --elastic-dsn=<string> --cache-path=<string> [dir_path]
"
}

fn print_usage() {
    println!("\n{}\n", usage());
}

/// Returns Info from command line args
///
/// # Arguments
///
/// * `args` - A Vector string from command line
///
/// # Examples
///
/// ```
/// use oms::app::commands::info;
/// use std::collections::HashMap;
/// 
/// let args = vec!["oms".to_string(), "info".to_string(), "/home/me/text.txt".to_string()];
/// info::build_cmd(&args, HashMap::new());
/// ```
/// 
/// # Features
/// 
/// * [ ] PDF metadata (title, author, keywords...)
/// * [ ] PDf summary?
/// * [ ] Movie
/// 
pub fn build_cmd(args: &Vec<String>, options: HashMap<String, String>) -> Result<Info, io::Error> {
    let file_path = get_args_parameter(
        args,
        args.len() - 1, // Get last agruments
        "\ninfo error: 'file_path' parameter required\n"
    ).unwrap_or_default();
    
    if file_path.is_empty() {
        return Err(Error::new(
            ErrorKind::InvalidInput, 
            format!("\ninfo error: 'file_path' parameter required\n")
        ));
    }

    Ok(Info {
        file_path: file_path.to_string(),
        cmd_options: options,
    })
}
