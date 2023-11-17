mod info;
mod read;
mod search;

use std::io::{self, Error, ErrorKind};


pub trait Runnable {
    fn run(&self) -> Result<(), io::Error>;
}

pub fn parse_action(args: &Vec<String>) -> Result<Box<dyn Runnable>, io::Error> {
    let action = match get_args_parameter(args, 1, "") {
        Ok(v) => v,
        Err(_) => "info".to_string(), // Default action: info
    };
    
    match &action[..] {
        "info" => Ok(Box::new(info::build_action()?)),
        "read" => Ok(Box::new(read::build_action(args)?)),
        "search" => Ok(Box::new(search::build_action(args)?)),
        _ => Err(Error::new(
            ErrorKind::InvalidInput, 
            format!("'{action}' is not a valid command{}", info::help_command())
        ))
    } 
}

fn get_args_parameter(args: &Vec<String>, index:usize, error_message: &str) -> Result<String, io::Error> {
    let parameter = match args.get(index) {
        Some(v) => v,
        None => return Err(Error::new(
            ErrorKind::InvalidInput, 
            format!("{error_message}")
        ))
    };
    Ok(parameter.to_string())
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
        if let Err(err) = parse_action(&vec!["cmd".to_string(), "unknown command".to_string()]) {
            assert!(err.to_string().contains("unknown command"), "Error should contain 'unknown command'");
            return Ok(());
        }
        Err(format!("parse_action should throw unkown command"))
    }
}
