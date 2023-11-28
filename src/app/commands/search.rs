use std::io;
use std::fs;
use std::sync::mpsc::{self, Sender};
use std::thread;
use super::{get_args_parameter, Runnable};
use crate::helpers::string;

/// # Search command
/// 
/// Search for a term in a file or directory
/// 
/// ## Usage
/// 
/// `cargo run -- search /home/solofo/Videos/text.txt you`
/// `oms search /home/solofo/Videos/text.txt you`
/// 
/// ## Features
/// 
/// * [x] Search in text file: OK
/// * [ ] Search in PDF: TODO
/// * [ ] Search in office file: TODO
/// * [ ] Search in directory: TODO
/// * [ ] Search in movie: TODO
/// * [ ] Search in link: TODO
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

        search_text_file(&self.file_path, &self.search_term, &tx);

        for message in rx {
            println!("{message}");
        }

        println!("\n");
        Ok(())
    }
}

fn search_text_file(file_path: &String, search_term: &String, tx: &Sender<String>) {
    let tx = tx.clone();
    let file_path = file_path.clone();
    let content =  fs::read_to_string(&file_path).expect("Unable to read file").clone();
    let search_term = search_term.clone();

    let mut result = String::new();
    thread::spawn(move || {
        string::search_lines(&content, &search_term)
            .enumerate()
            .for_each(|(index, (line, text))| {
                if index == 0 {
                    result.push_str(&format!("\n{}\nLine(s) found for \"{}\":\n\n", file_path, search_term));
                }
                result.push_str(&format!("l.{}\t->\t{}\n", line, text));
            });
        match tx.send(result) {
            _ => (),
        };
    });
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
