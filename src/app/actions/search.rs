use std::io;
use super::{get_args_parameter, Runnable};
use crate::helpers::{file, string};

/**
* cargo run -- search /home/solofo/Videos/text.txt you
*/

pub struct Search {
    file_path: String,
    search_term: String,
}

impl Runnable for Search {
    fn run(&self) -> Result<(), io::Error> {
        let content =  file::get_content(&self.file_path)?;

        println!("\nLine(s) found for \"{}\":\n", self.search_term);
        for line  in string::search_lines(&content, &self.search_term) {
            println!("{}\t{}", line.0, line.1);
        }
        println!("\n");

        Ok(())
    }
}

pub fn build_action(args: &Vec<String>) -> Result<Search, io::Error> {
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
