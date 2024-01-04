pub mod commands;

use std::{env, error::Error, process};
use commands::{Runnable, parse_command};
use crate::helpers::input::parse_command_option;


/// App version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

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
    /// use oms::app::App;
    /// let app = App::from_env().unwrap();
    /// ```
    pub fn from_env() -> Result<App, Box<dyn Error>> {
        Self::from_args(&env::args().collect())
    }

    /// Create App from a string vector
    /// 
    /// # Examples
    /// 
    /// ```
    /// use oms::app::App;
    /// let cmd = vec!["oms".to_string(), "help".to_string()];
    /// let app = App::from_args(&cmd).unwrap();
    /// ```
    pub fn from_args(args: &Vec<String>) -> Result<App, Box<dyn Error>> {
        let options = parse_command_option(args);
        if options.contains_key("version") {
            println!("{VERSION}");
            process::exit(0);
        }
        Ok(App {
            action: parse_command(args, options)?,
        })
    }

    /// Run the Runnable Action of the App
    /// 
    /// # Examples
    /// 
    /// ```
    /// use oms::app::App;
    /// let cmd = vec!["oms".to_string(), "help".to_string()];
    /// let app = App::from_args(&cmd).unwrap();
    /// app.run();
    /// ```
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        self.action.run()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{io:: {Error, ErrorKind}, cell::RefCell, rc::Rc};
    use super::{App, commands::Runnable};

    #[test]
    fn from_args() {
        assert!(super::App::from_args(&vec![]).is_ok(), "from args should be ok");
    }

    #[test]
    fn run_success() {
        struct MockRunnable {
            counter: Rc<RefCell<u8>>,
        }
        impl Runnable for MockRunnable {
            fn run(&self) -> Result<(), Error> {
                *self.counter.borrow_mut() += 1;
                Ok(())
            }
        }

        let counter = Rc::new(RefCell::new(0));

        let mock_runnable = MockRunnable {
            counter: Rc::clone(&counter),
        };
        
        let app: App = App {
            action: Box::new(mock_runnable)
        };
        assert!(app.run().is_ok(), "App run 1 should be successfull");
        assert!(app.run().is_ok(), "App run 2 should be successfull");
        assert_eq!(2, *counter.borrow(), "App should run twice");
    }

    #[test]
    fn run_error() {
        struct MockRunnable {}
        impl Runnable for MockRunnable {
            fn run(&self) -> Result<(), Error> {
                Err(Error::new(ErrorKind::WouldBlock, "WouldBlock"))
            }
        }

        let mock_runnable = MockRunnable {};
        let app = App {
            action: Box::new(mock_runnable)
        };
        match app.run() {
            Ok(_) => panic!("App run should failed"),
            Err(err) => assert_eq!(err.to_string(), "WouldBlock", "App run should fail with message 'WouldBlock'"),
        };
    }
}
