use std::{io::{Error, ErrorKind}, cmp::max};
use crate::app::commands::OPTION_SEPARATOR;

type Result<T> = std::result::Result<T, std::io::Error>;


pub struct SearchOption {
    pub search_term: String,

    pub display: String,
    pub thread: usize,

    pub extensions: Vec<String>,
    pub exclude_extensions: Vec<String>,

    pub files: Vec<String>,
    pub exclude_files: Vec<String>,
    pub provider: String,
}

impl SearchOption {
    pub fn new(search_term: String) -> Self {
        SearchOption {
            search_term: search_term,
            display: String::from("all"),
            thread: max(1, num_cpus::get() - 1),
            extensions: vec![],
            exclude_extensions: vec![],
            files: vec![],
            exclude_files: vec![],
            provider: String::from("local"),
        }
    }
    
    pub fn set_provider(&mut self, value: &str) -> Result<()> {
        match value {
            "local" | "api" => {
                self.provider = value.to_string();
                Ok(())
            },
            _ => Err(Error::new(
                ErrorKind::NotFound, 
                format!("Unknown value for provider")
            ))
        }
    }

    pub fn set_thread(&mut self, value: &String) -> Result<()> {
        match value.parse::<usize>() {
            Ok(v) => {
                self.thread = max(1, v);
                Ok(())
            },
            _ => Err(Error::new(
                ErrorKind::NotFound, 
                format!("Invalid value for pause")
            ))
        }
    }

    pub fn set_display(&mut self, value: &String) -> Result<()> {
        match value.as_str() {
            "file-only" | "debug" => {
                self.display = value.clone();
                Ok(())
            },
            _ => Err(Error::new(
                ErrorKind::NotFound, 
                format!("Unknown value for display")
            ))
        }
    }

    pub fn extensions_from(&mut self, value: &String) -> Result<()> {
        self.extensions = value.split(OPTION_SEPARATOR).map(|s| s.to_lowercase().to_string()).collect();
        Ok(())
    }

    pub fn exclude_extensions_from(&mut self, value: &String) -> Result<()> {
        self.exclude_extensions = value.split(OPTION_SEPARATOR).map(|s| s.to_lowercase().to_string()).collect();
        Ok(())
    }

    pub fn files_from(&mut self, value: &String) -> Result<()> {
        self.files = value.split(OPTION_SEPARATOR).map(|s| s.to_lowercase().to_string()).collect();
        Ok(())
    }

    pub fn exclude_files_from(&mut self, value: &String) -> Result<()> {
        self.exclude_files = value.split(OPTION_SEPARATOR).map(|s| s.to_lowercase().to_string()).collect();
        Ok(())
    }
    
    pub fn has_extension(&self, extension: &String) -> bool {
        if self.extensions.len() == 0 {
            return true;
        }
        self.extensions.contains(&extension.to_lowercase())
    }

    pub fn is_extension_excluded(&self, extension: &String) -> bool {
        if self.exclude_extensions.len() == 0 {
            return false;
        }
        if extension.is_empty() {
            return false;
        }
        self.exclude_extensions.contains(extension)
    }

    pub fn has_file(&self, file_name: &String) -> bool {
        if self.files.len() == 0 {
            return true;
        }
        if file_name.is_empty() {
            return true;
        }
        self.files.contains(file_name)
    }

    pub fn is_file_excluded(&self, file_name: &String) -> bool {
        if self.exclude_files.len() == 0 {
            return false;
        }
        if file_name.is_empty() {
            return false;
        }
        self.exclude_files.contains(file_name)
    }
}

impl Clone for SearchOption {
    fn clone(&self) -> Self {
        SearchOption { 
            search_term: self.search_term.clone(),
            display: self.display.clone(),
            thread: self.thread,
            extensions: self.extensions.clone(),
            exclude_extensions: self.exclude_extensions.clone(),
            files: self.files.clone(),
            exclude_files: self.exclude_files.clone(),
            provider: self.provider.clone(),
        }
    }
}
