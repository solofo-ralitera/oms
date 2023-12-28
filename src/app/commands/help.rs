use std::{io, collections::HashMap};
use super::{Runnable, info, read, search, mserv};


/// # Help command
/// 
/// Display help for this app
/// 
/// ## Usage
/// 
/// `oms help`
pub struct Help {}

impl Runnable for Help {
    /// Start processing the command
    fn run(&self) -> Result<(), io::Error> {
        println!("\
\nUsage:  oms [options] <command> [<args>]

Available commands:

{}
{}
{}
{}
{}
",
        self::usage(),
        info::usage(),
        read::usage(),
        search::usage(),
        mserv::usage(),
);
        Ok(())
    }
}

/// Help message for this command
pub fn usage() -> &'static str {
    "help                    Display these informations"
}

/// Help message to display in case of error
pub fn help_command() -> &'static str {
    "\nSee 'oms help'\n"
}

/// Returns Help
///
/// # Examples
///
/// ```
/// use oms::app::commands::help;
/// use std::collections::HashMap;
/// 
/// help::build_cmd(&vec![], HashMap::new());
/// ```
pub fn build_cmd(_: &Vec<String>, _: HashMap<String, String>) -> Result<Help, io::Error> {
    Ok(Help {})
}
