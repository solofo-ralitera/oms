mod config;

use std::{io, env};
use config::{AppConfigAction, parse_action};

pub struct AppConfig {
    action: AppConfigAction,
}

impl AppConfig {
    fn new(action: AppConfigAction) -> Self {
        return Self {
            action: action,
        };
    }

    pub fn run(&self) -> Result<&Self, io::Error> {
        self.action.run()?;
        Ok(self)
    }
}


pub fn parse_from_env() -> Result<AppConfig, io::Error> {
    self::parse_args(&env::args().collect())
}

pub fn parse_args(args: &Vec<String>) -> Result<AppConfig, io::Error> {
    let action = parse_action(args)?;
    Ok(AppConfig::new(action))
}
