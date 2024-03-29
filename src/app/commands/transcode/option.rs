use std::{io::{Error, ErrorKind}, cmp::max, collections::HashMap};
use crate::{app::commands::OPTION_SEPARATOR, helpers::file};

type Result<T> = std::result::Result<T, std::io::Error>;

pub struct TranscodeOption {
    pub extensions: Vec<String>,
    pub thread: usize,
    pub delete: bool,
    pub keep_smallest: bool,
    pub split: usize,
    pub force: bool,
    pub check: bool,
    pub list: Vec<String>,
    pub skip_list: Vec<String>,
    output_formats: HashMap<String, String>,
}

impl TranscodeOption {
    pub fn new() -> Self {
        TranscodeOption {
            extensions: vec![],
            thread: max(1, num_cpus::get() - 1),
            delete: false,
            keep_smallest: false,
            split: 0,
            force: false,
            check: false,
            output_formats: HashMap::new(),
            list: vec![],
            skip_list: vec![],
        }
    }

    pub fn set_delete(&mut self) {
        self.delete = true;
    }

    pub fn set_keep_smallest(&mut self) {
        self.keep_smallest = true;
    }
    
    pub fn set_check(&mut self) {
        self.check = true;
    }

    pub fn set_force(&mut self) {
        self.force = true;
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

    pub fn set_split(&mut self, value: &String) -> Result<()> {
        let value = if value.is_empty() {
            String::from("10")
        } else {
            value.to_string()
        };
        match value.parse::<usize>() {
            Ok(v) => {
                self.split = v;
                Ok(())
            },
            _ => Err(Error::new(
                ErrorKind::NotFound, 
                format!("Invalid value for split")
            )),
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

    pub fn set_list(&mut self, value: &String) -> Result<()> {
        if let Some(lines) = file::read_lines(value) {
            for (_, line) in lines.enumerate() {
                if let Ok(l) = line {
                    if !l.is_empty() {
                        self.list.push(l.trim().to_string());
                    }
                }
            }
            return Ok(());
        }
        return Err(Error::new(
            ErrorKind::NotFound, 
            format!("Unknown value for list")
        ));
    }

    pub fn set_skiplist(&mut self, value: &String) -> Result<()> {
        if let Some(lines) = file::read_lines(value) {
            for (_, line) in lines.enumerate() {
                if let Ok(l) = line {
                    if !l.is_empty() {
                        self.skip_list.push(file::get_file_name(&l.trim().to_string()));
                    }
                }
            }
            return Ok(());
        }
        return Err(Error::new(
            ErrorKind::NotFound, 
            format!("Unknown value for skip-list")
        ));
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

impl Clone for TranscodeOption {
    fn clone(&self) -> Self {
        TranscodeOption { 
            delete: self.delete,
            keep_smallest: self.keep_smallest,
            extensions: self.extensions.clone(),
            output_formats: self.output_formats.clone(),
            split: self.split,
            thread: self.thread,
            force: self.force,
            check: self.check,
            list: self.list.clone(),
            skip_list: self.list.clone(),
        }
    }
}
