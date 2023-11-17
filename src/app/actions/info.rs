use std::io;

use super::Runnable;

/**
* cargo run -- info
* cargo run
*/

pub struct Info {}

impl Runnable for Info {
    fn run(&self) -> Result<(), io::Error> {
        println!("");
        println!("Usage:  oms [OPTIONS] COMMAND");
        println!("");
        println!("Available commands:");
        println!("  info                    Display these informations");
        println!("  read [file_path]        Display the content of the file");
        Ok(())
    }
}

pub fn help_command() -> &'static str {
    "\nSee 'oms info'\n"
}

pub fn build_action() -> Result<Info, io::Error> {
    Ok(Info {})
}
