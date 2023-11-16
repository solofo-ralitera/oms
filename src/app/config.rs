mod info;
mod read;

use std::io::{self, Error, ErrorKind};

pub enum AppConfigAction {
    Info(),
    Read(String),
}

impl AppConfigAction {
    pub fn run(&self) -> Result<(), io::Error> {
        match &self {
            AppConfigAction::Info() => info::run(),
            AppConfigAction::Read(file_path) => read::run(&file_path),
        }
    }
}

fn get_args_parameter(args: &Vec<String>, index:usize, error_message: String) -> Result<String, io::Error> {
    let parameter = match args.get(index) {
        Some(v) => v,
        None => return Err(Error::new(
            ErrorKind::InvalidInput, 
            format!("{error_message}")
        ))
    };
    Ok(parameter.to_string())
}

pub fn parse_action(args: &Vec<String>) -> Result<AppConfigAction, io::Error> {
    let default_action = "info".to_string();
    let action = match args.get(1) {
        Some(v) => v,
        None => &default_action,
    };
    
    match &action[..] {
        "info" => Ok(AppConfigAction::Info()),
        "read" => {
            Ok(AppConfigAction::Read(get_args_parameter(
                args,
                2,
                format!("{}{}", read::error_command(), info::help_command())
            )?))
        },
        _ => Err(Error::new(
            ErrorKind::InvalidInput, 
            format!("\n'{action}' is not a valid command{}", info::help_command())
        ))
    } 
}

