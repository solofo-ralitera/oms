use std::{io::Error, thread::{self, JoinHandle}, collections::HashMap};
use super::{get_args_parameter, Runnable};
use std::fs;

type Result<T> = std::result::Result<T, std::io::Error>;

/// # Read command
/// 
/// Display the content of the given file
/// 
/// ## Usage
/// 
/// `oms read /home/solofo/Videos/text.txt`
/// `cargo run -- read ./Cargo.toml`
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
    /// Command options
    cmd_options: HashMap<String, String>
}

impl Runnable for Read {
    /// Start processing the command
    fn run(&self) -> Result<()> {
        // --help
        if self.cmd_options.contains_key("h") || self.cmd_options.contains_key("help") {
            print_usage();
            return Ok(());
        }

        match read_text_file(&self.file_path).join() {
            _ => Ok(()),
        }
    }
}

fn read_text_file(file_path: &String) -> JoinHandle<Result<()>> {
    let file_path = file_path.clone();
    thread::spawn(move || {
        match fs::read_to_string(file_path) {
            Ok(content) => {
                println!("{content}");
                Ok(())
            }
            Err(err) => {
                println!("\nRead error: {err}\n");
                print_usage();
                return Err(Error::new(
                    err.kind(), 
                    format!("{err}")
                ));
            }
        }
    })
}

/// Help message for this command
pub fn usage() -> &'static str {
    "\
read [file_path]        Display the content of the file
"
}

fn print_usage() {
    println!("\n{}\n", usage());
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
/// use std::collections::HashMap;
/// 
/// let args = vec!["oms".to_string(), "read".to_string(), "/home/me/text.txt".to_string()];
/// read::build_cmd(&args, HashMap::new());
/// ```
pub fn build_cmd(args: &Vec<String>, options: HashMap<String, String>) -> Result<Read> {
    let file_path = get_args_parameter(
        args,
        args.len() - 1, // Get last agruments
        "\nread error: 'file_path' parameter required\n"
    ).unwrap_or_default();
    
    Ok(Read {
        file_path: file_path.to_string(),
        cmd_options: options,
    })
}
