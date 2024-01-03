use std::{io::{Error, ErrorKind}, cmp::max};
use crate::app::commands::OPTION_SEPARATOR;

type Result<T> = std::result::Result<T, std::io::Error>;

pub struct TranscodeOption {
    pub extensions: Vec<String>,
    pub thread: usize,
    pub delete: bool,
}

impl TranscodeOption {
    pub fn new() -> Self {
        TranscodeOption {
            extensions: vec![],
            thread: max(1, num_cpus::get() - 1),
            delete: false,
        }
    }

    pub fn set_delete(&mut self) {
        self.delete = true;
    }

    pub fn set_thread(&mut self, value: &String) -> Result<()> {
        match value.parse::<usize>() {
            Ok(v) => {
                self.thread = v;
                Ok(())
            },
            _ => Err(Error::new(
                ErrorKind::NotFound, 
                format!("Invalid value for thread")
            ))
        }
    }

    pub fn extensions_from(&mut self, value: &String) -> Result<()> {
        self.extensions = value.split(OPTION_SEPARATOR).map(|s| s.to_lowercase().to_string()).collect();
        Ok(())
    }

    pub fn has_extension(&self, extension: &String) -> bool {
        if self.extensions.len() == 0 {
            return true;
        }
        self.extensions.contains(&extension.to_lowercase())
    }
}