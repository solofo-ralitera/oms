pub mod text;
pub mod pdf;


use std::io::{self, Error, ErrorKind};
use std::fs::{metadata, read_dir};
use std::path::Path;
use std::sync::mpsc::{self, Sender};
use std::thread;
use super::{get_args_parameter, Runnable};
use crate::helpers::file;

/// # Search command
/// 
/// Search for a term in a file or directory
/// 
/// ## Usage
/// 
/// `cargo run -- search ./Cargo.toml opt`
/// `cargo run -- search . opt`
/// 
/// ## Features
/// 
/// * [x] Search in text file: OK
/// * [o] Search in PDF: TODO
/// * [ ] Search in office file: TODO
/// * [x] Search in directory: TODO
/// * [ ] Search in movie: TODO
/// * [ ] Search in link: TODO
/// * [ ] Search arguments: extension, ...
/// *  ...
/// 
pub struct Search {
    /// path of the file to search in
    file_path: String,
    /// The search term
    search_term: String,
}

impl Runnable for Search {
    /// Start processing the command
    fn run(&self) -> Result<(), io::Error> {
        let (tx, rx) = mpsc::channel();

        match metadata(&self.file_path) {
            Ok(md) => {
                if md.is_file() {
                    search_in_file(&self.file_path, &self.search_term, tx.clone());
                } else if md.is_dir() {
                    search_in_dir(&self.file_path, &self.search_term, tx.clone());
                } else {
                    return Err(Error::new(
                        ErrorKind::InvalidInput, 
                        format!("\n{}\n\tread error: unknown file\n\n", self.file_path)
                    ));
                }
            },
            Err(err) => {
                return Err(Error::new(
                    ErrorKind::NotFound, 
                    format!("\n{}\n\tread error: {}\n\n", self.file_path, err)
                ));
            }
        };

        drop(tx);
        for message in rx {
            println!("{message}");
        }
        Ok(())
    }
}


fn search_in_dir(dir_path: &String, search_term: &String, tx: Sender<String>) {
    let dir_path = dir_path.clone();
    let search_term = search_term.clone();

    thread::spawn(move || {
        for entry in read_dir(Path::new(&dir_path)).unwrap() {
            let path = entry.unwrap().path();
            if path.is_file() {
                search_in_file(&path.to_str().unwrap().to_string(), &search_term, tx.clone())
            } else if path.is_dir() {
                search_in_dir(&path.to_str().unwrap().to_string(), &search_term, tx.clone())
            }
        }
    });
}

fn search_in_file(file_path: &String, search_term: &String, tx: Sender<String>) {
    match file::get_extension(file_path).unwrap_or("").to_lowercase().as_str() {
        "pdf" => pdf::search_in_file(file_path, search_term, tx),
        _ => text::search_in_file(file_path, search_term, tx),
    }

}

fn text_contains(text: &String, search_term: &String) -> bool {
    text.to_lowercase().contains(search_term)
}

fn format_file_display(file_path: &String) -> String {
    format!("\n{}\n", file_path)
}

fn format_line_found<'a>(item: &'a String, text: &'a String) -> String {
    format!("  {} ->  {}\n", item, text.trim())
}

/// Help message for this command
pub fn usage() -> &'static str {
    "\
    search [file_path] [query]      Search in file
                                    Display each line of the file containing the query text
    "
}


/// Returns Search from command line args
///
/// # Arguments
///
/// * `args` - A Vector string from command line
///
/// # Examples
///
/// ```
/// use oms::app::commands::search;
/// let args = vec!["oms".to_string(), "search".to_string(), "/home/me/text.txt".to_string(), "search term".to_string()];
/// search::build_cmd(&args);
/// ```
pub fn build_cmd(args: &Vec<String>) -> Result<Search, io::Error> {
    let file_path = get_args_parameter(
        args,
        2,
        "\nread error: 'file_path' parameter required\n"
    )?;

    let search_term = get_args_parameter(
        args,
        3,
        "\nread error: 'search_term' parameter required\n"
    )?;
    
    Ok(Search {
        file_path: file_path.to_string(),
        search_term: search_term.to_string(),
    })
}
