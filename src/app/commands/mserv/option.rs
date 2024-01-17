use std::{io, net::{SocketAddr, ToSocketAddrs}, cmp::max};
use crate::helpers::{db::elastic::Elastic, rtrim_char, file};

type Result<T> = std::result::Result<T, std::io::Error>;

pub struct MservOption {
    pub base_path: String,
    pub urls: Vec<SocketAddr>,
    pub elastic: Option<Elastic>,
    pub provider: String,
    pub transcode_output: String,
    pub transcode_thread: usize,
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
            provider: String::from("api"),
            transcode_output: String::from("mp4"),
            transcode_thread: max(1, num_cpus::get() - 1),
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

    pub fn set_provider(&mut self, value: &str) -> Result<()> {
        match value {
            "local" | "api" => {
                self.provider = value.to_string();
                Ok(())
            },
            _ => Err(io::Error::new(
                io::ErrorKind::NotFound, 
                format!("Unknown value for provider")
            ))
        }
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

    pub fn set_transcode_output(&mut self, value: &String) {
        self.transcode_output = value.to_string();
    }

    pub fn set_transcode_thread(&mut self, value: &String) -> Result<()> {
        match value.parse::<usize>() {
            Ok(v) => {
                self.transcode_thread = v;
                Ok(())
            },
            _ => Err(io::Error::new(
                io::ErrorKind::NotFound, 
                format!("Invalid value for transcode thread")
            ))
        }
    }

    pub fn set_basepath(&mut self, value: &String) -> Result<()> {
        match file::check_dir(value) {
            Ok(_) => {
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
        self.elastic = Some(Elastic::new(value)?);
        return Ok(());
    }
}


impl Clone for MservOption {
    fn clone(&self) -> Self {
        MservOption { 
            base_path: self.base_path.clone(),
            urls: self.urls.clone(),
            elastic: self.elastic.clone(),
            provider: self.provider.clone(),
            transcode_output: self.transcode_output.clone(),
            transcode_thread: self.transcode_thread,
        }
    }
}
