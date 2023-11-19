use std::io;
use super::{Runnable, info, read, search};


/// # Help command
/// 
/// Display help for this app
/// 
/// ## Usage
/// `oms help`
pub struct Help {}

impl Runnable for Help {
    fn run(&self) -> Result<(), io::Error> {
        println!("\
\nUsage:  oms [OPTIONS] COMMAND

Available commands:
    {}
    {}
    {}
    {}
",
        self::usage(),
        info::usage(),
        read::usage(),
        search::usage(),
);
        Ok(())
    }
}

pub fn usage() -> &'static str {
    "help                    Display these informations"
}

pub fn help_command() -> &'static str {
    "\nSee 'oms help'\n"
}

/// Returns Help
pub fn build_cmd() -> Result<Help, io::Error> {
    Ok(Help {})
}
