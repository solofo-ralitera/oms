mod actions;

use std::{env, error::Error};
use actions::{Runnable, parse_action};

pub struct App {
    action: Box<dyn Runnable>,
}

impl App {
    fn new(action: Box<dyn Runnable>) -> Self {
        App {
            action: action,
        }
    }

    pub fn from_env() -> Result<App, Box<dyn Error>> {
        Self::from_args(&env::args().collect())
    }

    pub fn from_args(args: &Vec<String>) -> Result<App, Box<dyn Error>> {
        let action = parse_action(args)?;
        Ok(App::new(action))
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
