use std::fs;
use std::io::{self, Error, ErrorKind};

/// Check if the given file exists
///
/// # Arguments
///
/// * `file_path` - A String ref that holds the path of the file to check
/// 
/// # Examples
/// 
/// ```
/// let file_path = "/home/me/text.txt".to_string();
/// match check_file_path(file_path) {
///     Ok(v) => v,
///     Err(err) => return Err(io::Error::new(
///         ErrorKind::InvalidInput, 
///         format!("{err}")
///     ))
/// };
/// ```
pub fn check_file_path(file_path: &String) -> Result<&String, io::Error> {
    match fs::metadata(file_path) {
        Ok(_) => Ok(file_path),
        Err(err) => Err(err),
    }
}

/// Get th content of the givent file
/// 
/// # Arguments
///
/// * `file_path` - A String ref that holds the path of the file to check
///
/// # Examples
/// 
/// ```
/// let file_path = "/home/me/text.txt".to_string();
/// let content =  file::get_content(&file_path)?;
/// ```
/// 
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
