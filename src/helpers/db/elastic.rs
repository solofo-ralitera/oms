use std::io;
use serde::Serialize;
use url::Url;
use crate::helpers::{http, rtrim_char};


pub struct Elastic {
    pub url: String,
}

impl Elastic {
    pub fn new(url: &String) -> Result<Self, io::Error> {
        match Url::parse(url) {
            Ok(elastic_url) if (elastic_url.path().is_empty() || elastic_url.path().eq("/")) => return Err(io::Error::new(
                io::ErrorKind::InvalidInput, 
                format!("Missing index in the url {url}, ")
            )),
            Ok(_) => return Ok(Self {
                url: rtrim_char(url, '/'),
            }),
            _ => return Err(io::Error::new(
                io::ErrorKind::InvalidInput, 
                format!("Invalid elastic url {url}")
            ))
        }
    }

    pub fn insert<T: Serialize>(&self, id: &String, body: &T) -> String {
        let query;
        if id.is_empty() {
            query = http::post_body(
                &format!("{}/_doc/", self.url), 
                "POST",
                &vec![], 
                body
            );
        } else {
            query = http::post_body(
                &format!("{}/_doc/{id}", self.url), 
                "PUT",
                &vec![], 
                body
            );
        }
        match query {
            Ok(result) => result,
            Err(err) => err.to_string(),
        }
    }
}

impl Clone for Elastic {
    fn clone(&self) -> Self {
        Elastic {
            url: self.url.clone(),
        }
    }
}
