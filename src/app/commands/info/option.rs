use std::io::{Error, ErrorKind};
use crate::helpers::{file, db::elastic::Elastic};

type Result<T> = std::result::Result<T, std::io::Error>;

pub struct InfoOption {
    pub provider: String,
    pub list: Vec<String>,
    pub display_preview: bool,
    pub elastic: Option<Elastic>,
}

impl InfoOption {
    pub fn new() -> Self {
        InfoOption {
            provider: "local".to_string(),
            list: vec![],
            display_preview: true,
            elastic: None,
        }
    }

    pub fn set_provider(&mut self, value: &String) -> Result<()> {
        match value.as_str() {
            "local" | "tmdb" | "omdb" => {
                self.provider = value.clone();
                Ok(())
            },
            _ => Err(Error::new(
                ErrorKind::NotFound, 
                format!("Unknown value for provider")
            ))
        }
    }

    pub fn hide_preview(&mut self) {
        self.display_preview = false;
    }

    pub fn set_list(&mut self, value: &String) -> Result<()> {
        let lines = file::read_lines(value).enumerate();
        for (_, line) in lines {
            if let Ok(l) = line {
                self.list.push(l);
            }
        }
        Ok(())
    }

    pub fn set_elastic(&mut self, value: &String) {
        self.elastic = Some(Elastic::new(&"oms", value));
    }

}

impl Clone for InfoOption {
    fn clone(&self) -> Self {
        InfoOption { 
            provider: self.provider.clone(),
            list: self.list.clone(),
            display_preview: self.display_preview.clone(),
            elastic: self.elastic.clone(),
        }
    }
}
