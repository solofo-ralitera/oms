use std::io;
use super::{get_args_parameter, Runnable};
use crate::helpers::{file, string};

/**
* cargo run -- search /home/solofo/Videos/text.txt you
*   Search in text file: OK
*   Search in PDF: TODO
*   Search in office file: TODO
*   Search in directory: TODO
*   Search in movie: TODO
*   Search in link: TODO
*   ...
*/

pub struct Search {
    file_path: String,
    search_term: String,
}

impl Runnable for Search {
    fn run(&self) -> Result<(), io::Error> {
        let content =  file::get_content(&self.file_path)?;

        println!("\n{}\nLine(s) found for \"{}\":\n", self.file_path, self.search_term);

        string::search_lines(&content, &self.search_term)
            .iter()
            .for_each(|line| {
                println!("{}\t{}", line.0, line.1);
            });

        println!("\n");
        Ok(())
    }
}

pub fn usage() -> &'static str {
    "\
    search [file_path] [query]      Search in file
                                    Display each line of the file containing the query text
    "
}

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
