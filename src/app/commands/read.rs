use std::io::{self, Error};
use super::{get_args_parameter, Runnable};
use std::fs;

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
/// * [x] Read text file: OK
/// * [ ] Read pdf: TODO
/// * [ ] Read movie: TODO (?)
/// * [ ] Read office file: TODO
/// 
pub struct Read {
    /// the path of the file to read
    file_path: String,
}

impl Runnable for Read {
    /// Start processing the command
    fn run(&self) -> Result<(), io::Error> {
        match fs::read_to_string(&self.file_path) {
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

/// Help message for this command
pub fn usage() -> &'static str {
    "read [file_path]        Display the content of the file"
}

/// Returns Read from command line args
///
/// # Arguments
///
/// * `args` - A Vector string from command line
///
/// # Examples
///
/// ```
/// use oms::app::commands::read;
/// let args = vec!["oms".to_string(), "read".to_string(), "/home/me/text.txt".to_string()];
/// read::build_cmd(&args);
/// ```
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
