use std::{io, collections::HashMap};
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
    /// Command options
    cmd_options: HashMap<String, String>,
}

impl Runnable for Info {
      /// Start processing the command
     fn run(&self) -> Result<(), std::io::Error> {
        
        // --help
        if self.cmd_options.contains_key("h") || self.cmd_options.contains_key("help") {
            print_usage();
            return Ok(());
        }
        
        println!("    WIP {}", self.file_path);
        Ok(())
    }
}

/// Help message for this command
pub fn usage() -> &'static str {
    "\
info [file_path]        Display file informations
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
/// * [ ] Movie
/// 
pub fn build_cmd(args: &Vec<String>, options: HashMap<String, String>) -> Result<Info, io::Error> {
    let file_path = get_args_parameter(
        args,
        args.len() - 1, // Get last agruments
        "\nread error: 'file_path' parameter required\n"
    ).unwrap_or_default();
    
    Ok(Info {
        file_path: file_path.to_string(),
        cmd_options: options,
    })
}
