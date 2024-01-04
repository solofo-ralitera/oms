mod pdf;
mod video;
mod image;
mod option;
mod audio;

use std::{io::{self, Error, ErrorKind}, collections::HashMap, fs, path::Path, sync::{mpsc::{self, Sender}, Arc, Mutex}};
use once_cell::sync::Lazy;

use crate::helpers::{file::{get_extension, get_file_name, self}, threadpool::ThreadPool};
use super::{Runnable, get_args_parameter};
use self::{pdf::PdfInfo, video::VideoInfo, option::InfoOption, image::ImageInfo, audio::AudioInfo};


/// # Info command
/// 
/// Finds information related to the file
/// 
/// ## Usage
///
/// `oms info /home/me/video.mp4`
/// 
/// cargo run -- info --elastic-url="http://localhost:9200" --cache-path="/media/solofo/MEDIA/.oms" --thread=5 "/media/solofo/MEDIA/films/"
/// 
/// 
/// ## Features
/// 
/// * [ ] File information: TODO
/// * [ ] Video information: TODO
/// * [ ] Read office file: TODO
/// 
pub struct Info {
    /// the path of the file
    pub file_path: String,
    /// Command options
    pub cmd_options: HashMap<String, String>,
}

static INFO_RUNNING: Lazy<Arc<Mutex<bool>>> = Lazy::new(|| {
    return Arc::new(Mutex::new(false));
});

impl Runnable for Info {
      /// Start processing the command
     fn run(&self) -> Result<(), std::io::Error> {
        let b_isrunning = Arc::clone(&INFO_RUNNING);
        let mut b_isrunning = b_isrunning.lock().unwrap();

        if *b_isrunning == true {
            return Err(Error::new(
                ErrorKind::AddrInUse, 
                format!("\nInfo is already running\n")
            ));
        }
        *b_isrunning = true;

        let (tx, rx) = mpsc::channel();
        let mut info_option = InfoOption::new();
        let mut file_path = self.file_path.to_string();
        // --help
        if self.cmd_options.contains_key("h") || self.cmd_options.contains_key("help") {
            *b_isrunning = false;
            print_usage();
            return Ok(());
        }

        info_option.set_basepath(&file_path)?;
        for (option, value) in &self.cmd_options {
            match option.as_str() {
                "p" | "provider" => info_option.set_provider(value)?,
                "hide-preview" => info_option.hide_preview(),
                "elastic-url" => info_option.set_elastic(value)?,
                "t" | "thread" => info_option.set_thread(value)?,
                "list" => { 
                    info_option.set_list(value)?; // Files are provided in option
                    file_path.clear(); // Ignore the file in last option
                },
                arg => {
                    *b_isrunning = false;
                    return Err(Error::new(
                        ErrorKind::InvalidInput, 
                        format!("\nUnkown argument {}\n", arg)
                    ));                    
                },
            };
        }
        
        let thread_pool = ThreadPool::new(info_option.thread);

        // if files are provided in option as list
        if info_option.list.len() > 0 {
            file_info_from_list(&info_option, &thread_pool, tx);
            *b_isrunning = false;
            return Ok(());
        }

        match fs::metadata(&file_path) {
            Ok(md) if md.is_dir() => {
                dir_info(&file_path, &info_option, &thread_pool, tx.clone());
            },
            Ok(md) if md.is_file() => {
                file_info(&file_path, &info_option, &thread_pool, tx.clone());
            },
            _ => file_info(&file_path, &info_option, &thread_pool, tx.clone()),
        };

        drop(tx);
        for message in rx {
            if info_option.display_preview == true {
                println!("{message}");
            }
        }

        *b_isrunning = false;
        Ok(())
    }
}

fn dir_info(dir_path: &String, info_option: &InfoOption, thread_pool: &ThreadPool, tx: Sender<String>) {
    for entry in fs::read_dir(Path::new(&dir_path)).unwrap() {
        let path = entry.unwrap().path();
        if path.is_file() {
            file_info(&path.to_str().unwrap().to_string(), &info_option, thread_pool, tx.clone())
        } else if path.is_dir() {
            dir_info(&path.to_str().unwrap().to_string(), &info_option, thread_pool, tx.clone())
        }
    }
}

fn file_info(file_path: &String, info_option: &InfoOption, thread_pool: &ThreadPool, tx: Sender<String>)  {
    let file_path = file_path.clone();
    let info_option = info_option.clone();
    thread_pool.execute(move || {
        let extension = get_extension(&file_path).to_lowercase();
        let extension = extension.as_str();
        
        if file::PDF_EXTENSIONS.contains(&extension) {
            PdfInfo {
                file_path: &file_path,
                info_option: &info_option,
            }.info(tx);
        }
        else if file::IMAGE_EXTENSIONS.contains(&extension) {
            ImageInfo {
                file_path: &file_path,
                info_option: &info_option,
            }.info(tx);
        }
        else if file::VIDEO_EXTENSIONS.contains(&extension) || extension.is_empty() {
            VideoInfo { 
                video_raw_name: &get_file_name(&file_path),
                file_path: &file_path,
                info_option: &info_option,
            }.info(tx);
        }
        else if file::AUDIO_EXTENSIONS.contains(&extension) || extension.is_empty() {
            AudioInfo { 
                file_path: &file_path,
                info_option: &info_option,
            }.info(tx);
        }
        else if file::VIDEO_EXTENSIONS_IGNORED.contains(&extension) {
            ();
        } else {
            print!("\n{file_path}: Format not supported\n");
        }
    });
}

fn file_info_from_list(info_option: &InfoOption, thread_pool: &ThreadPool, tx: Sender<String>) {
    for file_path in &info_option.list {
        match fs::metadata(&file_path) {
            Ok(md) if md.is_dir() => {
                dir_info(file_path, info_option, thread_pool, tx.clone());
            },
            Ok(md) if md.is_file() => {
                file_info(file_path, info_option, thread_pool, tx.clone());
            },
            _ => (),
        };
    }
}

/// Help message for this command
pub fn usage() -> &'static str {
    "\
info [OPTIONS] <file_path/dir_path>
    Display/get file informations
    --help
    -p <string> --provider=<string>   possible value: local, api (default)
    --cache-path=<string>   Cache path, default ./.oms/
    --elastic-url=<string>  Elastic search server
    --hide-preview=<bool>   Mute display
    --list=<sting>          Path of a file containing the list of files to parse

    For videos: info --elastic-url=<string> --cache-path=<string> [dir_path]
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
/// * [ ] Video
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
