use std::{collections::HashMap, fs, io, path::Path};
use colored::Colorize;
use crate::helpers::{media::video::{self, title::VideoTitle}, input, file};
use super::{Runnable, get_args_parameter};

type Result<T> = std::result::Result<T, std::io::Error>;

/// # Rename video file to "Title (year)"
/// 
pub struct RenameMovie {
    /// the path of the file to rename
    file_path: String,
    cmd_options: HashMap<String, String>,
}

impl Runnable for RenameMovie {
    fn run(&self) -> Result<()> {
        // --help
        if self.cmd_options.contains_key("h") || self.cmd_options.contains_key("help") {
            print_usage();
            return Ok(());
        }
        let mut rename_option = RenameMovieOption::new();

        for (option, value) in &self.cmd_options {
            match option.as_str() {
                "p" | "provider" => rename_option.set_provider(value)?,
                arg => return Err(io::Error::new(
                    io::ErrorKind::InvalidInput, 
                    format!("\nUnkown argument {}\n", arg)
                )),
            };
        }

        match fs::metadata(&self.file_path) {
            Ok(md) if md.is_dir() => rename_dir(&self.file_path, &rename_option),
            Ok(md) if md.is_file() => rename_file(&self.file_path, &rename_option),
            _ => return Err(io::Error::new(
                io::ErrorKind::AddrInUse, 
                format!("\nCannot rename file {}\n", self.file_path)
            )),
        };
        Ok(())
    }
}

impl RenameMovieOption {
    pub fn new() -> Self {
        RenameMovieOption {
            provider: String::from("local"),
        }
    }

    pub fn set_provider(&mut self, value: &str) -> Result<()> {
        match value {
            "local" | "api" => {
                self.provider = value.to_string();
                Ok(())
            },
            _ => Err(io::Error::new(
                io::ErrorKind::NotFound, 
                format!("Unknown value for provider")
            ))
        }
    }
}
pub struct RenameMovieOption {
    pub provider: String,
}

fn rename_dir(dir_path: &String, rename_option: &RenameMovieOption) {
    for entry in fs::read_dir(Path::new(&dir_path)).unwrap() {
        let path = entry.unwrap().path();
        if path.is_file() {
            rename_file(&path.to_str().unwrap().to_string(), rename_option)
        } else if path.is_dir() {
            rename_dir(&path.to_str().unwrap().to_string(), rename_option)
        }
    }
}

fn get_title_year_from_provider(file_path: &String, file_name: &String) -> Result<(String, String)> {
    let videos = video::result::get_video_result(
            &file_path,
            &String::new(),
            &String::from("api")
        ).unwrap_or(vec![]);
    // If many movies correspond to the given title: let the user choose the corresponding one
    if videos.len() > 1 {
        println!("{} movies found for \"{}\":", videos.len(), file_name.blue());
        for (idx, video) in videos.iter().enumerate() {
            println!("  {idx} {} ({})", video.title, video.year);
        }
        let movie_index = input::read_line("Choose the appropriate movie number (leave empty to skip this file): ");
        if movie_index.is_empty() {
            return Ok((String::new(), String::new()));
        }
        match movie_index.parse::<usize>() {
            Ok(index) if videos.get(index).is_some() => {
                return Ok((format!("{}", videos[index].title), format!("{}", videos[index].year)));
            },
            _ => return Err(io::Error::new(
                io::ErrorKind::InvalidInput, 
                "Invalid index"
            )),
        }
    }
    // If one result found from provider, it's the right one? 
    else if videos.len() == 1 {
        return Ok((format!("{}", videos[0].title), format!("{}", videos[0].year)));
    }
    else {
        // Normally never used, provider fallback to local if api not found
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput, 
            format!("No information from providers found for {}", file_name)
        ))
    }    
}

fn rename_file(file_path: &String, rename_option: &RenameMovieOption) {
    if !file::is_video_file(file_path) {
        return;
    }
    let file_name = file::get_file_name(file_path);
    let extenstion = file::get_extension(&file_name).to_lowercase();

    let video_title = VideoTitle::from(&file_name);

    // Check if the current file is already in the new format
    let check_new_name = format!("{} ({}).{extenstion}", video_title.title, video_title.year);
    if file_name.to_lowercase().eq(&check_new_name.to_lowercase()) {
        return;
    }

    let mut movie_title = format!("{}", video_title.title);
    let mut movie_year = format!("{}", video_title.year);

    // If api: get title and year from provider
    if rename_option.provider == "api" {
        match get_title_year_from_provider(&file_path, &file_name) {
            Ok((title, year)) if !title.is_empty() && !year.is_empty() => {
                movie_title = title;
                movie_year = year;
            },
            Err(err) => {
                print!("{}", err.to_string().red());
            },
            _ => (),
        }
    }

    if movie_year == "0" || movie_year.is_empty() {
        println!("{} {}", "Skip".blue().bold(), file_name.blue());
        return;
    }

    let new_name = format!("{movie_title} ({movie_year}).{}", extenstion.to_lowercase());
    
    // Check agin if the current file is alread in the new format
    if file_name.to_lowercase().eq(&new_name.to_lowercase()) {
        return;
    }
    if let Err(err) = file::rename_file(file_path, &new_name) {
        println!("{} {}: {}", "Unable to rename".red().bold(), file_path.red(), err.to_string().red());
    } else {
        println!("{} {} -> {}", "Rename".green().bold(), file_name.green(), new_name.green());
    }
}

/// Help message for this command
pub fn usage() -> &'static str {
    "\
renamemovie [OPTIONS] <file_path/dir_path>
Rename video file to \"Title (year)\"
    
--help
-p <string> --provider=<string>   allowed value: local (default), api
--cache-path=<string>   Cache path
"
}

fn print_usage() {
    println!("\n{}\n", usage());
}

pub fn build_cmd(args: &Vec<String>, options: HashMap<String, String>) -> Result<RenameMovie> {
    let file_path = get_args_parameter(
        args,
        args.len() - 1, // Get last agruments
        "\nrenameMovie error: 'file_path' parameter required\n"
    ).unwrap_or_default();
    
    Ok(RenameMovie {
        file_path: file_path.to_string(),
        cmd_options: options,
    })
}
