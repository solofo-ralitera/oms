use std::{io::{Error, ErrorKind}, cmp::max, collections::HashMap};
use crate::{app::commands::OPTION_SEPARATOR, helpers::file};

type Result<T> = std::result::Result<T, std::io::Error>;

pub struct TranscodeOption {
    pub extensions: Vec<String>,
    pub thread: usize,
    pub delete: bool,
    output_formats: HashMap<String, String>,
}

impl TranscodeOption {
    pub fn new() -> Self {
        TranscodeOption {
            extensions: vec![],
            thread: max(1, num_cpus::get() - 1),
            delete: false,
            output_formats: HashMap::new(),
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

    /// flv>webm,avi>mp4,mp4
    pub fn set_output(&mut self, value: &String) -> Result<()> {
        for extension in value.split(",").into_iter() {
            if extension.is_empty() {
                continue;
            }
            let mut a_extension = extension.split(">");
            let extension_from = a_extension.next().unwrap_or("");
            let extension_to = a_extension.next().unwrap_or("");
            if extension_from.is_empty() {
                continue;
            }
            let from;
            let to;
            if extension_to.is_empty() {
                // If no >, its the default output format
                from = String::from("*");
                to = extension_from.to_lowercase().to_string();
            } else {
                from = extension_from.to_string();
                to = extension_to.to_lowercase().to_string();
            }
            if file::VIDEO_EXTENSIONS.contains(&to.as_str()) {
                self.output_formats.insert(from, to);
            } else {
                return Err(Error::new(
                    ErrorKind::NotFound, 
                    format!("Invalid value for output")
                ));
            }
        }
        Ok(())
    }

    pub fn get_output(&self, extension: &String) -> String {
        if let Some(output) = self.output_formats.get(&extension.to_lowercase()) {
            return output.to_string();
        }
        if let Some(output) = self.output_formats.get(&String::from("*")) {
            return output.to_string();
        }
        return String::from("mp4");
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