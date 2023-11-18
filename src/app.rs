mod commands;

use std::{env, error::Error};
use commands::{Runnable, parse_command};

pub struct App {
    action: Box<dyn Runnable>,
}

impl App {
    pub fn from_env() -> Result<App, Box<dyn Error>> {
        Self::from_args(&env::args().collect())
    }

    pub fn from_args(args: &Vec<String>) -> Result<App, Box<dyn Error>> {
        Ok(App {
            action: parse_command(args)?,
        })
    }

    pub fn run(&self) -> Result<&Self, Box<dyn Error>> {
        self.action.run()?;
        Ok(self)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn from_args() -> Result<(), String> {
        if let Err(err) = super::App::from_args(&vec![]) {
            return Err(err.to_string());
        }
        Ok(())
    }
}
