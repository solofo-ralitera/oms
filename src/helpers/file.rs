use std::ffi::OsStr;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use bytes::Bytes;
use image::EncodableLayout;
use mime::Mime;
use ring::digest::{Context, SHA256};
use data_encoding::HEXUPPER;


type Result<T> = std::result::Result<T, std::io::Error>;

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
pub fn check_file(file_path: &str) -> Result<&str> {
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

pub fn read_buf(file_path: &str) -> Bytes {
   let mut buf = Vec::new();
   if let Ok(mut file) = File::open(file_path) {
      file.read_to_end(&mut buf).unwrap(); 
      return Bytes::from(buf);
   }
   return Bytes::from("");
}

pub fn get_extension(filename: &str) -> String {
    Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
        .unwrap_or("")
        .to_string()
}

pub fn get_mimetype(file_path: &str) -> Mime {
   let parts : Vec<&str> = file_path.split('.').collect();
   let res = match parts.last() {
      Some(v) => match *v {
         "ico" | "png" => mime::IMAGE_PNG,
         "jpeg" | "jpg" => mime::IMAGE_JPEG,
         "json" => mime::APPLICATION_JSON,
         "js" => mime::TEXT_JAVASCRIPT,
         "pdf" => mime::APPLICATION_PDF,
         "html" => mime::TEXT_HTML_UTF_8,
         &_ => mime::TEXT_PLAIN,
      },
      None => mime::TEXT_PLAIN,
   };
   return res;
}

pub fn remove_extension(filename: &str) -> String {
   return Path::new(filename)
      .file_stem()
      .unwrap()
      .to_str()
      .unwrap_or("")
      .to_string();
}

pub fn get_file_name(file_path: &String) -> String {
    Path::new(file_path)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap_or("")
        .to_string()
}

pub fn write_file_content(file_name: &Path, content: &str, append: bool) -> Result<()> {
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

 pub fn write_file_bytes(file_name: &Path, content: &Bytes) -> Result<()> {
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

 pub fn sha256(file_path: &String) -> Result<String> {
   if let Ok(output) = Command::new("sha256sum").arg(file_path).stdout(Stdio::piped()).output() {
      if let Ok(stdout) = String::from_utf8(output.stdout) {
         let res = stdout.split("  ").into_iter().next().unwrap_or_default();
         if !res.is_empty() {
            return Ok(res.trim().to_string());
         }
      }
   }

   let input = File::open(file_path)?;
    let mut reader = BufReader::new(input);

   let mut context = Context::new(&SHA256);
   let mut buffer = [0; 1024];

   loop {
       let count = reader.read(&mut buffer)?;
       if count == 0 {
           break;
       }
       context.update(&buffer[..count]);
   }

   return Ok(HEXUPPER.encode(context.finish().as_ref()));
 }