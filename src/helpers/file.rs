use std::fs;
use std::io::{self, Error, ErrorKind};

pub fn check_file_path(file_path: &String) -> Result<&String, io::Error> {
    match fs::metadata(file_path) {
        Ok(_) => Ok(file_path),
        Err(err) => Err(err),
    }
}


pub fn get_content(file_path: &String) -> Result<String, io::Error> {
    let file_path = match check_file_path(file_path) {
        Ok(v) => v,
        Err(err) => return Err(io::Error::new(
            ErrorKind::InvalidInput, 
            format!("{err}")
        ))
    };
    match fs::read_to_string(file_path) {
        Ok(content) => Ok(content),
        Err(err) => return Err(Error::new(
            ErrorKind::PermissionDenied, 
            format!("{err}")
        ))
    }    
}
