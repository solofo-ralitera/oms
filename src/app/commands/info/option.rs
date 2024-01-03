use std::{io::{Error, ErrorKind}, cmp::max};
use crate::helpers::{file, db::elastic::Elastic, rtrim_char};

type Result<T> = std::result::Result<T, std::io::Error>;

pub struct InfoOption {
    pub base_path: String,
    pub list: Vec<String>,
    pub display_preview: bool,
    pub elastic: Option<Elastic>,
    pub thread: usize,
    pub provider: String,
}

impl InfoOption {
    pub fn new() -> Self {
        InfoOption {
            base_path: String::new(),
            list: vec![],
            display_preview: true,
            elastic: None,
            thread: max(1, num_cpus::get() - 1),
            provider: String::from("api"),
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

    pub fn set_basepath(&mut self, value: &String) -> Result<()> {
        match file::check_dir(value) {
            Ok(_) => {
                self.base_path = rtrim_char(value, '/').trim().to_string();
                return Ok(());
            },
            _ => Ok(()),
        }
    }

    pub fn hide_preview(&mut self) {
        self.display_preview = false;
    }

    pub fn set_list(&mut self, value: &String) -> Result<()> {
        if let Some(lines) = file::read_lines(value) {
            for (_, line) in lines.enumerate() {
                if let Ok(l) = line {
                    self.list.push(l);
                }
            }
            return Ok(());
        }
        return Err(Error::new(
            ErrorKind::NotFound, 
            format!("Unknown value for list")
        ));
    }

    pub fn set_elastic(&mut self, value: &String) -> Result<()> {
        self.elastic = Some(Elastic::new(value)?);
        return Ok(());
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
}

impl Clone for InfoOption {
    fn clone(&self) -> Self {
        InfoOption { 
            base_path: self.base_path.clone(),
            list: self.list.clone(),
            display_preview: self.display_preview.clone(),
            elastic: self.elastic.clone(),
            thread: self.thread,
            provider: self.provider.clone(),
        }
    }
}
