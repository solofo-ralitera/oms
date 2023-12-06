mod pdf;
mod movie;
mod option;

use std::{io::{self, Error, ErrorKind}, collections::HashMap, fs::{metadata, read_dir}, thread, path::Path, sync::mpsc::{self, Sender}};
use crate::helpers::file::get_extension;
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
/// ## Features
/// 
/// * [ ] File information: TODO
/// * [ ] Movie information: TODO
/// * [ ] Read office file: TODO
/// 
pub struct Info {
    /// the path of the file
    file_path: String,
    /// Command options
    cmd_options: HashMap<String, String>,
}

impl Runnable for Info {
      /// Start processing the command
     fn run(&self) -> Result<(), std::io::Error> {
        let (tx, rx) = mpsc::channel();
        let mut info_option = InfoOption::new();

        // --help
        if self.cmd_options.contains_key("h") || self.cmd_options.contains_key("help") {
            print_usage();
            return Ok(());
        }

        for (option, value) in &self.cmd_options {
            match option.as_str() {
                "provider" => info_option.set_provider(value)?,
                arg => {
                    return Err(Error::new(
                        ErrorKind::InvalidInput, 
                        format!("\nUnkown argument {}\n", arg)
                    ));
                },
            };
        }

        match metadata(&self.file_path) {
            Ok(md) => {
                if md.is_dir() {
                    dir_info(&self.file_path, &info_option, tx.clone());
                }
                else if md.is_file() {
                    file_info(&self.file_path, &info_option, tx.clone());
                }
            },
            _ => file_info(&self.file_path, &info_option, tx.clone()),
        };
        
        drop(tx);
        for message in rx {
            println!("{message}");
        }
        Ok(())
    }
}

fn dir_info(dir_path: &String, info_option: &InfoOption, tx: Sender<String>) {
    let dir_path = dir_path.clone();
    let info_option = info_option.clone();
    thread::spawn(move || {
        for entry in read_dir(Path::new(&dir_path)).unwrap() {
            let path = entry.unwrap().path();
            if path.is_file() {
                file_info(&path.to_str().unwrap().to_string(), &info_option, tx.clone())
            } else if path.is_dir() {
                dir_info(&path.to_str().unwrap().to_string(), &info_option, tx.clone())
            }
        }
    });
}

fn file_info(file_path: &String, info_option: &InfoOption, tx: Sender<String>)  {
    let file_path = file_path.clone();
    let info_option = info_option.clone();
    thread::spawn(move || {
        let extension = get_extension(&file_path).to_lowercase();
        match extension.as_str() {
            "pdf" => PdfInfo { file_path: &file_path}.info(tx),
            "torrent" | "mp4" => MovieInfo { 
                file_path: &file_path,
                info_option: &info_option,
            }.info(tx),
            _ => MovieInfo { 
                file_path: &file_path,
                info_option: &info_option,
            }.info(tx),
        }
    });
}

/// Help message for this command
pub fn usage() -> &'static str {
    "\
info [file_path]        Display file informations
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
        "\nread error: 'file_path' parameter required\n"
    ).unwrap_or_default();
    
    Ok(Info {
        file_path: file_path.to_string(),
        cmd_options: options,
    })
}
