use std::{io, net::{SocketAddr, ToSocketAddrs}, fs};
use crate::helpers::{db::elastic::Elastic, rtrim_char};

type Result<T> = std::result::Result<T, std::io::Error>;

pub struct MservOption {
    pub base_path: String,
    pub urls: Vec<SocketAddr>,
    pub elastic: Option<Elastic>,
}

impl MservOption {
    fn addr_from_string(url: &String) -> SocketAddr {
        return url.to_socket_addrs().unwrap().next().unwrap()
    }

    pub fn new() -> Self {
        MservOption {
            base_path: String::new(),
            urls: vec![Self::addr_from_string(&"localhost:7777".to_string())],
            elastic: None,
        }
    }

    pub fn display_urls(&self) -> String {
        return self.urls.iter().map(|addr| {
            let mut url = addr.to_string();
            if !url.starts_with("http") {
                url = String::from("http://") + &url;
            }
            url.push(' ');
            return url;
        }).collect();
    }

    pub fn set_url(&mut self, value: &String) -> Result<()> {
        if let Ok(addrs) = value.to_socket_addrs() {
            self.urls.clear();
            for addr in addrs {
                self.urls.push(addr);
            }
            return Ok(());
        } else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput, 
                format!("Invalid url {value}")
            ));
        }
    }

    pub fn set_basepath(&mut self, value: &String) -> Result<()> {
        match fs::metadata(value) {
            Ok(md) if md.is_dir() => {
                self.base_path = rtrim_char(value, '/').trim().to_string();
                return Ok(());
            },
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput, 
                format!("Base path {value} is not a directory")
            )),
        }
    }

    pub fn set_elastic(&mut self, value: &String) -> Result<()> {
        match Elastic::new(value) {
            Ok(elastic) => {
                self.elastic = Some(elastic);
                return Ok(());
            },
            Err(err) => return Err(io::Error::new(
                io::ErrorKind::InvalidInput, 
                format!("Invalid elastic url: {err}")
            ))
        }
    }
}


impl Clone for MservOption {
    fn clone(&self) -> Self {
        MservOption { 
            base_path: self.base_path.clone(),
            urls: self.urls.clone(),
            elastic: self.elastic.clone(),
        }
    }
}
