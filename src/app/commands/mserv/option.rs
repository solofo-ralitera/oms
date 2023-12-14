use std::io;
use url::Url;
use crate::helpers::db::elastic::Elastic;

type Result<T> = std::result::Result<T, std::io::Error>;

pub struct MservOption {
    pub url: Url,
    pub elastic: Option<Elastic>,
}

impl MservOption {
    pub fn new() -> Self {
        MservOption {
            url: Url::parse("http://127.0.0.1:7777").unwrap(),
            elastic: None,
        }
    }

    pub fn set_url(&mut self, value: &String) -> Result<()> {
        if let Ok(url) = Url::parse(value) {
            self.url = url;
            return Ok(());
        } else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput, 
                format!("INvalid url {value}")
            ));
        }
    }

    pub fn set_elastic(&mut self, value: &String) {
        self.elastic = Some(Elastic::new(value));
    }
}


impl Clone for MservOption {
    fn clone(&self) -> Self {
        MservOption { 
            url: self.url.clone(),
            elastic: self.elastic.clone(),
        }
    }
}
