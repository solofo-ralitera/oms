use std::io;
use regex::Regex;
use serde::Serialize;
use url::Url;
use crate::helpers::http;


pub struct Elastic {
    pub url: String,
}

impl Elastic {
    pub fn new(url: &String) -> Result<Self, io::Error> {
        match Url::parse(url) {
            Ok(elastic_url) if elastic_url.path().is_empty() => return Err(io::Error::new(
                io::ErrorKind::InvalidInput, 
                format!("Missing index in the url {url}, ")
            )),
            Ok(_) => {
                let re = Regex::new(r"/$").unwrap();
                return Ok(Self {
                    url: re.replace(url, ".").to_string(),
                });
            },
            _ => return Err(io::Error::new(
                io::ErrorKind::InvalidInput, 
                format!("Invalid elastic url {url}")
            ))
        }
    }

    pub fn insert<T: Serialize>(&self, body: &T) {
        let _ = http::post_body(
            &format!("{}/_doc/", self.url), 
            &vec![], 
            body
        );
    }
}

impl Clone for Elastic {
    fn clone(&self) -> Self {
        Elastic {
            url: self.url.clone(),
        }
    }
}
