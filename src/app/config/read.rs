use std::fs;
use std::io::{self, Error, ErrorKind};

fn check_file_path(file_path: &String) -> Result<&String, io::Error> {
    match fs::metadata(file_path) {
        Ok(_) => Ok(file_path),
        Err(err) => Err(err),
    }
}

pub fn error_command() -> &'static str {
    "\nread error: 'file_path' parameter required\n"
}

pub fn run(file_path: &String) -> Result<(), io::Error>{
    let file_path = match check_file_path(file_path) {
        Ok(v) => v,
        Err(err) => return Err(Error::new(
            ErrorKind::InvalidInput, 
            format!("\nread error: {err}\n")
        ))
    };
    match fs::read_to_string(file_path) {
        Ok(contents) => {
            println!("{contents}");
            Ok(())
        }
        Err(err) => return Err(Error::new(
            ErrorKind::PermissionDenied, 
            format!("\nread error: {err}\n")
        ))
    }
}
