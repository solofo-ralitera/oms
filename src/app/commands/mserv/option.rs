use std::{io, net::{SocketAddr, ToSocketAddrs}};
use crate::helpers::db::elastic::Elastic;

type Result<T> = std::result::Result<T, std::io::Error>;

pub struct MservOption {
    pub urls: Vec<SocketAddr>,
    pub elastic: Option<Elastic>,
}

impl MservOption {
    fn addr_from_string(url: &String) -> SocketAddr {
        return url.to_socket_addrs().unwrap().next().unwrap()
    }

    pub fn new() -> Self {
        MservOption {
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

    pub fn set_elastic(&mut self, value: &String) {
        self.elastic = Some(Elastic::new(value));
    }
}


impl Clone for MservOption {
    fn clone(&self) -> Self {
        MservOption { 
            urls: self.urls.clone(),
            elastic: self.elastic.clone(),
        }
    }
}
