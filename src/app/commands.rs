//! # Commands
//!
//! Provide all commands for oms app
//! 
//! like info, read, search...

pub mod help;
pub mod read;
pub mod search;
pub mod info;
pub mod mserv;

use std::{io::{Error, ErrorKind}, collections::HashMap};

type Result<T> = std::result::Result<T, std::io::Error>;

const OPTION_SEPARATOR: char = ',';

/// All commands must implement this Trait
pub trait Runnable {
    fn run(&self) -> Result<()>;
}

/// Parse the comand line
/// 
/// # Arguments
///
/// * `args` - A Vector string ref that holds arguments passed via the command line
/// 
/// # Examples
/// 
/// ```
/// use oms::app::commands::parse_command;
/// use std::collections::HashMap;
/// 
/// if let Ok(action) = parse_command(&vec!["cmd".to_string(), "help".to_string()], HashMap::new()) {
///     action.run();
/// } else {
///     panic!("Should be ok")
/// }
/// 
/// if let Err(err) = parse_command(&vec!["cmd".to_string(), "unknown_command".to_string()], HashMap::new()) {
///     assert!(err.to_string().contains("unknown_command"), "Error should contain 'unknown_command'");
/// } else {
///     panic!("Should be an Err")
/// }
/// ```
pub fn parse_command(args: &Vec<String>, options: HashMap<String, String>) -> Result<Box<dyn Runnable>> {
    let cmd = get_args_parameter(args, 1, "")
        .unwrap_or("help");
    
    match cmd {
        "help" => Ok(Box::new(help::build_cmd()?)),
        "read" => Ok(Box::new(read::build_cmd(args, options)?)),
        "search" => Ok(Box::new(search::build_cmd(args, options)?)),
        "info" => Ok(Box::new(info::build_cmd(args, options)?)),
        "mserv" => Ok(Box::new(mserv::build_cmd(options)?)),
        _ => Err(Error::new(
            ErrorKind::InvalidInput, 
            format!("'{cmd}' is not a valid command{}", help::help_command())
        ))
    }
}

/// Get item of a string vector a the givent index
/// 
/// # Arguments
///
/// * `args` - A Vector string ref where to get the value
/// * `index` - Index of the item to get
/// * `error_message` - A string slice that holds the error message if error occurs
/// 
/// # Examples
/// 
/// ```
/// use oms::app::commands::get_args_parameter;
/// let args = vec!["oms".to_string(), "info".to_string()];
/// 
/// let cmd1 = get_args_parameter(&args, 1, "").unwrap_or("help");
/// assert_eq!("info", cmd1);
/// 
/// let cmd2 = get_args_parameter(&args, 2, "").unwrap_or("index not found");
/// assert_eq!("index not found", cmd2);
/// ```
pub fn get_args_parameter<'a>(args: &'a Vec<String>, index:usize, error_message: &str) -> Result<&'a str> {
    let parameter = match args.get(index) {
        Some(v) => v,
        None => return Err(Error::new(
            ErrorKind::InvalidInput, 
            error_message
        ))
    };
    Ok(parameter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_args_info_single_parameter() {
        if let Ok(v) = get_args_parameter(&vec!["info".to_string()], 0, "") {
            assert_eq!(v, "info");
            return ();
        }
        panic!("get_args_parameter should return Ok(info)");
    }

    #[test]
    fn parse_args_info_two_parameters() {
        if let Ok(v) = get_args_parameter(&vec!["info".to_string(), "help".to_string()], 1, "") {
            assert_eq!(v, "help");
            return ();
        }
        panic!("get_args_parameter should return Ok(help)");
    }

    #[test]
    fn parse_args_info_error_message() {
        if let Err(err) = get_args_parameter(&vec!["info".to_string(), "help".to_string()], 3, "error message") {
            assert_eq!(err.to_string(), "error message");
            return ();
        }
        panic!("get_args_parameter should throw error 'error message'");
    }

    #[test]
    fn parse_action_error() {
        if let Err(err) = parse_command(
            &vec!["cmd".to_string(), "unknown command".to_string()],
            HashMap::new()
        ) {
            assert!(err.to_string().contains("unknown command"), "Error should contain 'unknown command'");
            return ();
        }
        panic!("parse_action should throw unkown command");
    }
}
