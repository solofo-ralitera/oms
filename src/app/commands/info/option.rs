use std::io::{Error, ErrorKind};

pub struct InfoOption {
    pub provider: String,
}

impl InfoOption {
    pub fn new() -> Self {
        InfoOption {
            provider: "local".to_string(),
        }
    }

    pub fn set_provider(&mut self, value: &String) -> Result<(), Error> {
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
}

impl Clone for InfoOption {
    fn clone(&self) -> Self {
        InfoOption { 
            provider: self.provider.clone(),
        }
    }
}
