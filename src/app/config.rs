mod info;
mod read;

use std::{env, io::{self, Error, ErrorKind}};

pub enum AppConfigAction {
    Info(),
    Read(String),
}

impl AppConfigAction {
    fn run(&self) -> Result<(), io::Error> {
        match &self {
            AppConfigAction::Info() => info::run(),
            AppConfigAction::Read(file_path) => read::run(&file_path),
        }
    }
}

pub struct AppConfig {
    action: AppConfigAction,
}

impl AppConfig {
    pub fn new(action: AppConfigAction) -> Self {
        return Self {
            action: action,
        };
    }

    pub fn run(&self) -> Result<&Self, io::Error> {
        self.action.run()?;
        Ok(self)
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

fn parse_action(args: &Vec<String>) -> Result<AppConfigAction, io::Error> {
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

pub fn parse_from_env() -> Result<AppConfig, io::Error> {
    self::parse_args(&env::args().collect())
}

pub fn parse_args(args: &Vec<String>) -> Result<AppConfig, io::Error> {
    let action = parse_action(args)?;
    Ok(AppConfig::new(action))
}
