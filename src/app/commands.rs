mod help;
mod read;
mod search;
mod info;

use std::io::{self, Error, ErrorKind};


pub trait Runnable {
    fn run(&self) -> Result<(), io::Error>;
}

pub fn parse_command(args: &Vec<String>) -> Result<Box<dyn Runnable>, io::Error> {
    let cmd = get_args_parameter(args, 1, "")
        .unwrap_or("help");
    
    match cmd {
        "help" => Ok(Box::new(help::build_cmd()?)),
        "read" => Ok(Box::new(read::build_cmd(args)?)),
        "search" => Ok(Box::new(search::build_cmd(args)?)),
        "info" => Ok(Box::new(info::build_cmd(args)?)),
        _ => Err(Error::new(
            ErrorKind::InvalidInput, 
            format!("'{cmd}' is not a valid command{}", help::help_command())
        ))
    } 
}

fn get_args_parameter<'a>(args: &'a Vec<String>, index:usize, error_message: &str) -> Result<&'a str, io::Error> {
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
    fn parse_args_info_single_parameter() -> Result<(), String> {
        if let Ok(v) = get_args_parameter(&vec!["info".to_string()], 0, "") {
            assert_eq!(v, "info");
            return Ok(());
        }
        Err(format!("get_args_parameter should return Ok(info)"))
    }

    #[test]
    fn parse_args_info_two_parameters() -> Result<(), String> {
        if let Ok(v) = get_args_parameter(&vec!["info".to_string(), "help".to_string()], 1, "") {
            assert_eq!(v, "help");
            return Ok(());
        }
        Err(format!("get_args_parameter should return Ok(help)"))
    }

    #[test]
    fn parse_args_info_error_message() -> Result<(), String> {
        if let Err(err) = get_args_parameter(&vec!["info".to_string(), "help".to_string()], 3, "error message") {
            assert_eq!(err.to_string(), "error message");
            return Ok(());
        }
        Err(format!("get_args_parameter should throw error 'error message'"))
    }

    #[test]
    fn parse_action_error() -> Result<(), String> {
        if let Err(err) = parse_command(&vec!["cmd".to_string(), "unknown command".to_string()]) {
            assert!(err.to_string().contains("unknown command"), "Error should contain 'unknown command'");
            return Ok(());
        }
        Err(format!("parse_action should throw unkown command"))
    }
}
