mod commands;

use std::{env, error::Error};
use commands::{Runnable, parse_command};

/// It's the main application
pub struct App {
    /// The action to execute, implementing Runnable Trait
    action: Box<dyn Runnable>,
}

impl App {
    /// Create App from the arguments that this program was started with (normally passed via the command line).
    /// 
    /// # Examples
    /// ```
    /// App::from_env.unwrap_or_else(|err| {
    ///     eprintln!("\n{err}");
    ///     process::exit(1);
    /// });
    /// ```
    /// 
    pub fn from_env() -> Result<App, Box<dyn Error>> {
        Self::from_args(&env::args().collect())
    }

    /// Create App from a string vector
    /// 
    /// # Examples
    /// ```
    /// let cms = vec!["oms".to_string(), "help".to_string()];
    /// let app = App::from_args(&cms)?;
    /// ```
    pub fn from_args(args: &Vec<String>) -> Result<App, Box<dyn Error>> {
        Ok(App {
            action: parse_command(args)?,
        })
    }

    /// Run the Runnable Action of the App
    /// # Examples
    /// ```
    /// les cms = vec!["oms".to_string(), "help".to_string()];
    /// let app = App::from_args(&cms)?.run()?;
    /// ```
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
