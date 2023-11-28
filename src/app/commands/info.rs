use std::io;
use super::{Runnable, get_args_parameter};


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
}

impl Runnable for Info {
      /// Start processing the command
     fn run(&self) -> Result<(), std::io::Error> {
        println!("    WIP {}", self.file_path);
        Ok(())
    }
}

/// Help message for this command
pub fn usage() -> &'static str {
    "info [file_path]        Display file informations"
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
/// let args = vec!["oms".to_string(), "info".to_string(), "/home/me/text.txt".to_string()];
/// info::build_cmd(&args);
/// ```
pub fn build_cmd(args: &Vec<String>) -> Result<Info, io::Error> {
    let file_path = get_args_parameter(
        args,
        2,
        "\nread error: 'file_path' parameter required\n"
    )?;
    
    Ok(Info {
        file_path: file_path.to_string(),
    })
}
