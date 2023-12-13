use std::io;

use url::Url;
use crate::helpers::db::elastic::Elastic;

type Result<T> = std::result::Result<T, std::io::Error>;

pub struct MservOption {
    pub url: String,
    pub elastic: Option<Elastic>,
}

impl MservOption {
    pub fn new() -> Self {
        MservOption {
            url: String::from("127.0.0.1:7777"),
            elastic: None,
        }
    }

    pub fn set_url(&mut self, value: &String) -> Result<()> {
        if let Ok(_) = Url::parse(value) {
            self.url = value.to_string();
            return Ok(());
        } else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput, 
                format!("INvalid url {value}")
            ))
        }
    }

    pub fn set_elastic(&mut self, value: &String) {
        self.elastic = Some(Elastic::new(value));
    }
}
