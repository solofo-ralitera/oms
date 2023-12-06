use std::ffi::OsStr;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use bytes::Bytes;
use image::EncodableLayout;


/// Check if the given file exists
///
/// # Arguments
///
/// * `file_path` - the path of the file to check
/// 
/// # Examples
/// 
/// ```
/// use oms::helpers::file;
/// 
/// match file::check_file("./Cargo.toml") {
///     Ok(file_path) => assert_eq!("./Cargo.toml", file_path),
///     Err(err) => panic!("Should be Ok"),
/// };
/// 
/// match file::check_file("./404.txt") {
///     Ok(file_path) => panic!("Should throw error"),
///     Err(err) => assert!(err.to_string().starts_with("No")),
/// };
/// ```
pub fn check_file(file_path: &str) -> Result<&str, io::Error> {
    match fs::metadata(file_path) {
        Ok(_) => Ok(file_path),
        Err(err) => Err(err),
    }
}

///
/// // https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
/// // https://linuxhint.com/rust-read-from-file-line-line/
/// 
pub fn read_lines(file_path: &str) -> io::Lines<io::BufReader<File>> {
    let file = File::open(file_path).expect(&format!("Unable to open file {file_path}"));
    let reader = BufReader::new(file);
    reader.lines()
}

pub fn get_extension(filename: &str) -> String {
    Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
        .unwrap_or("")
        .to_string()

}

pub fn get_file_name(file_path: &String) -> String {
    Path::new(file_path)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap_or("")
        .to_string()
}

pub fn write_file_content(file_name: &Path, content: &str, append: bool) -> Result<(), io::Error> {
    // TODO: auto create sub/directories
    let mut fopen = match OpenOptions::new()
       .append(append)
       .write(true)
       .create(true)
       .open(file_name) {
          Ok(file) => file,
          Err(err) => return Err(err),
       };
    match fopen.write(content.as_bytes()) {
       Ok(_) => Ok(()),
       Err(err) => return Err(err),
    }
 }

 pub fn write_file_bytes(file_name: &Path, content: &Bytes) -> Result<(), io::Error> {
    // TODO: auto create sub/directories
    let mut fopen = match OpenOptions::new()
       .write(true)
       .create(true)
       .open(file_name) {
          Ok(file) => file,
          Err(err) => return Err(err),
       };
    match fopen.write(&content.as_bytes()) {
       Ok(_) => Ok(()),
       Err(err) => return Err(err),
    }
 }