use std::io::{self, Error};
use super::{get_args_parameter, Runnable};
use crate::helpers::file;


/// # Read command
/// 
/// Display the content of the given file
/// 
/// ## Usage
/// 
/// `oms read /home/me/texe.txt`
/// 
/// ## Features
/// 
/// * Read text file: OK
/// * Read pdf: TODO
/// * Read movie: TODO (?)
/// * Read office file: TODO
/// 
pub struct Read {
    /// the path of the file to read
    file_path: String,
}

impl Runnable for Read {
    fn run(&self) -> Result<(), io::Error> {
        match file::get_content(&self.file_path) {
            Ok(content) => {
                println!("{content}");
                Ok(())
            }
            Err(err) => return Err(Error::new(
                err.kind(), 
                format!("\nread error: {err}\n")
            ))
        }
    }
}

pub fn usage() -> &'static str {
    "read [file_path]        Display the content of the file"
}

pub fn build_cmd(args: &Vec<String>) -> Result<Read, io::Error> {
    let file_path = get_args_parameter(
        args,
        2,
        "\nread error: 'file_path' parameter required\n"
    )?;
    
    Ok(Read {
        file_path: file_path.to_string(),
    })
}
