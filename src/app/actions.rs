mod info;
mod read;

use std::io::{self, Error, ErrorKind};

pub enum Action {
    Info(),
    Read(String),
}

impl Action {
    pub fn run(&self) -> Result<(), io::Error> {
        match &self {
            Action::Info() => info::run(),
            Action::Read(file_path) => read::run(&file_path),
        }
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

pub fn parse_action(args: &Vec<String>) -> Result<Action, io::Error> {
    let action = match get_args_parameter(args, 1, "") {
        Ok(v) => v,
        Err(_) => "info".to_string(), // Default action: info
    };
    
    match &action[..] {
        "info" => Ok(Action::Info()),
        "read" => {
            Ok(Action::Read(get_args_parameter(
                args,
                2,
                &format!("{}{}", read::error_command(), info::help_command())[..]
            )?))
        },
        _ => Err(Error::new(
            ErrorKind::InvalidInput, 
            format!("\n'{action}' is not a valid command{}", info::help_command())
        ))
    } 
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
        Err(format!("get_args_parameter should throw error"))
    }

    #[test]
    fn parse_action_info() -> Result<(), String> {
        if let Ok(action) = parse_action(&vec!["cmd".to_string(), "info".to_string()]) {
            if let Action::Info() = action {
                return Ok(());
            }
        }
        Err(format!("parse_action should return Ok(Acion::Info)"))
    }

    #[test]
    fn parse_action_read() -> Result<(), String> {
        if let Ok(action) = parse_action(&vec!["cmd".to_string(), "read".to_string(), "filename".to_string()]) {
            if let Action::Read(_) = action {
                return Ok(());
            }
        }
        Err("parse_action should return Ok(Acion::Read)".to_string())
    }
}
