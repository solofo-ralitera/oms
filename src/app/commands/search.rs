pub mod option;
pub mod text;
pub mod pdf;
pub mod ms;
pub mod movie;


use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::fs;
use std::path::Path;
use std::sync::mpsc::{self, Sender};
use std::cmp;

use self::movie::MovieSearch;

use super::{get_args_parameter, Runnable, OPTION_SEPARATOR};
use crate::helpers::output::colorize;
use crate::helpers::threadpool::ThreadPool;
use crate::helpers::file::{self, get_file_name, get_extension};
use colored::Colorize;
use diacritics::remove_diacritics;
use option::SearchOption;
use pdf::PdfSearch;
use regex::Regex;
use text::TextSearch;
use ms::MsSearch;

type Result<T> = std::result::Result<T, std::io::Error>;

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
/// * [o] Search in file
///     * [x] text file
///     * [o] pdf: TODO: ?Identity-H Unimplemented?
///     * [x] office file
///         * [x] docx
///         * [x] xlsx
///         * [x] pptx
///     * [x] Search in movie
/// * [x] Search in directory
/// * [o] Search arguments
///     * [x] extensions
///     * [x] exlude file
///     * [x] exlude extension
///     * [x] display file-only
///     * [x] thread
pub struct Search {
    /// path of the file to search in
    file_path: String,
    /// The search term
    search_term: String,
    /// Command options
    cmd_options: HashMap<String, String>,
}

impl Runnable for Search {
    /// Start processing the command
    fn run(&self) -> Result<()> {
        let (tx, rx) = mpsc::channel();
        let mut search_option = SearchOption::new(self.search_term.clone());

        // --help
        if self.cmd_options.contains_key("h") || self.cmd_options.contains_key("help") {
            println!("\n{}\n", usage());
            return Ok(());
        }

        for (option, value) in &self.cmd_options {
            match option.as_str() {
                "display" => search_option.set_display(value)?,
                "t" | "thread" => search_option.set_thread(value)?,
                "e" | "extensions" => search_option.extensions_from(value)?,
                "exclude-extensions" => search_option.exclude_extensions_from(value)?, 
                "f" | "files" => search_option.files_from(value)?, 
                "exclude-files" => search_option.exclude_files_from(value)?, 
                arg => {
                    return Err(Error::new(
                        ErrorKind::InvalidInput, 
                        format!("\nUnkown argument {}\n", arg)
                    ));
                },
            };
        }

        let thread_pool = ThreadPool::new(search_option.thread);

        match fs::metadata(&self.file_path) {
            Ok(md) if md.is_file() => {
                search_in_file(&self.file_path, &self.search_term, &search_option, &thread_pool, tx.clone());
            },
            Ok(md) if md.is_dir() => {
                search_in_dir(&self.file_path, &self.search_term, &search_option, &thread_pool, tx.clone());
            },
            Ok(_) => return Err(Error::new(
                ErrorKind::InvalidInput, 
                format!("\n{}\nsearch error: unknown file\n\n", self.file_path)
            )),
            Err(err) => {
                return Err(Error::new(
                    ErrorKind::NotFound, 
                    format!("\n{}\nread error: {}\n\n", self.file_path, err)
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


fn search_in_dir(dir_path: &String, search_term: &String, search_option: &SearchOption, thread_pool: &ThreadPool, tx: Sender<String>) {
    let dir_path = dir_path.clone();
    let search_term = search_term.clone();
    let search_option = search_option.clone();

    for entry in fs::read_dir(Path::new(&dir_path)).unwrap() {
        let path = entry.unwrap().path();
        if path.is_file() {
            search_in_file(&path.to_str().unwrap().to_string(), &search_term, &search_option, thread_pool, tx.clone())
        } else if path.is_dir() {
            search_in_dir(&path.to_str().unwrap().to_string(), &search_term, &search_option, thread_pool, tx.clone())
        }
    }
}

fn search_in_file(file_path: &String, search_term: &String, search_option: &SearchOption, thread_pool: &ThreadPool, tx: Sender<String>) {
    let file_path = file_path.clone();
    let search_term = remove_diacritics(&search_term.to_lowercase().clone());
    let search_option = search_option.clone();

    let extension = get_extension(&file_path).to_lowercase();
    let file_name = get_file_name(&file_path).to_lowercase();
    
    if !search_option.has_extension(&extension) {
        return ();
    }
    if search_option.is_extension_excluded(&extension) {
        return ();
    }
    if !search_option.has_file(&file_name) {
        return ();
    }        
    if search_option.is_file_excluded(&file_name) {
        return ();
    }
    
    thread_pool.execute(move || {
        let extension = extension.as_str();
        if file::PDF_EXTENSIONS.contains(&extension) {
            PdfSearch { 
                file_path: &file_path, 
                search_term: &search_term, 
                search_option: &search_option,
            }.search(tx);
        }
        else if file::VIDEO_EXTENSIONS.contains(&extension) {
            MovieSearch {
                file_path: &file_path, 
                search_term: &search_term, 
                search_option: &search_option,
            }.search(tx);
        }
        else if file::MS_EXTENSIONS.contains(&extension) {
            MsSearch { 
                file_path: &file_path, 
                search_term: &search_term, 
                search_option: &search_option,
            }.search(tx);
        }
        else {
            TextSearch {
                file_path: &file_path,
                search_term: &search_term,
                search_option: &search_option,
            }.search(tx);
        }
    });
}

fn text_reg_contains(content: &String, search_term: &String) -> Option<Vec<String>> {
    let str_re = format!(r"(?im)(.{{0,100}}{}.{{0,100}})", search_term);
    let re = Regex::new(str_re.as_str()).unwrap();
    let mut results = vec![];
    re
        .captures_iter(&remove_diacritics(content))
        .map(|c| c.extract())
        .for_each(|(_, [c])| results.push(c.to_string()));
    if results.len() == 0 {
        return None;
    }
    Some(results)
}

fn format_file_display(file_path: &String) -> String {
    format!("{}\n", file_path.underline())
}

fn format_line_found<'a>(item: &'a String, text: &'a String, search_option: &SearchOption) -> String {
    if search_option.display == "file-only" {
        return "".to_string();
    }
    let re = Regex::new(format!(r"(?im){}", search_option.search_term).as_str()).unwrap();
    let output = format!(
        "{} -> {}\n",
        item,
        text.replace("\n", " ").trim().get(..cmp::min(500, text.len())).unwrap_or(text).trim()
    );
    colorize(&output, &re, (0, 102, 51)).unwrap_or(output)
}

/// Help message for this command
pub fn usage() -> String {
    format!("\
search [options] <file_path|directory_path> <query>
    Search in file or directory. Display each line of the file containing the query text
    --help    
    -e <string> --extensions=<string>    Search only in these file extensions, separated by '{OPTION_SEPARATOR}'
    --exclude-extensions=<string>    exlude these file extensions, separated by '{OPTION_SEPARATOR}'
    -f <> --files=<string>  Search only in these file names
    --exclude-files=<string>    exlude these files, separated by '{OPTION_SEPARATOR}'
    --display=<string>  file-only|debug
    -t <int> --thread=<int>    Max thread number, default 1
")
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
/// use std::collections::HashMap;
/// 
/// let args = vec!["oms".to_string(), "search".to_string(), "/home/me/text.txt".to_string(), "search term".to_string()];
/// search::build_cmd(&args, HashMap::new());
/// ```
pub fn build_cmd(args: &Vec<String>, options: HashMap<String, String>) -> Result<Search> {
    let file_path = get_args_parameter(
        args,
        args.len() - 2, // Get before last agruments
        "\nread error: 'file_path' parameter required\n"
    ).unwrap_or_default();

    let search_term = get_args_parameter(
        args,
        args.len() - 1, // Get last agruments
        "\nread error: 'search_term' parameter required\n"
    ).unwrap_or_default();
    
    Ok(Search {
        file_path: file_path.to_string(),
        search_term: search_term.to_string(),
        cmd_options: options,
    })
}
