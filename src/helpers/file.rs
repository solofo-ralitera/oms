use std::ffi::OsStr;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Read, Write, Seek, SeekFrom};
use std::path::Path;
use std::process::{Command, Stdio};
use bytes::Bytes;
use image::EncodableLayout;
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


pub fn file_size(file_path: &str) -> Result<u64> {
   match fs::metadata(file_path) {
      Ok(m) => Ok(m.len()),
      Err(err) => Err(err),
  }
}

// https://stackoverflow.com/questions/68694399/most-idiomatic-way-to-read-a-range-of-bytes-from-a-file
pub fn read_range(file_path: &str, start: u64, length: u64 ) -> Option<Vec<u8>> {
   if let Ok(mut f) = File::open(file_path) {
      if let Ok(_) = f.seek(SeekFrom::Start(start as u64)) {
         let mut buf = vec![0; length as usize];
         if  let Ok(_) = f.read_exact(&mut buf) {
            drop(f);
            return Some(buf);
         }
      }
      drop(f);
   }
   return None;
}

///
/// // https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
/// // https://linuxhint.com/rust-read-from-file-line-line/
pub fn read_lines(file_path: &str) -> Option<io::Lines<io::BufReader<File>>> {
   if let Ok(file) = File::open(file_path) {
      let reader = BufReader::new(file);
      return Some(reader.lines());
   }
   return None;
}

pub fn read_buf(file_path: &str) -> Vec<u8> {
   let mut buf = Vec::new();
   if let Ok(mut file) = File::open(file_path) {
      file.read_to_end(&mut buf).unwrap(); 
      return buf;
   }
   return vec![];
}

pub fn get_extension(filename: &str) -> String {
   Path::new(filename)
      .extension()
      .and_then(OsStr::to_str)
      .unwrap_or("")
      .to_string()
}

pub fn get_mimetype(file_path: &str) -> String {
   let guess = mime_guess::from_path(file_path);
   return guess.first().unwrap().to_string();
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


#[cfg(test)]
mod test {
   use super::*;

   #[test]
   fn file_get_extension_ok() {
      let file_name = String::from("test.txt");
      let extension = get_extension(&file_name);
      assert_eq!("txt", extension);

      let file_name = String::from("test");
      let extension = get_extension(&file_name);
      assert_eq!("", extension);
   }

   #[test]
   fn file_remove_extension_ok() {
      let file_name = String::from("test.txt");
      let extension = remove_extension(&file_name);
      assert_eq!("test", extension);

      let file_name = String::from("test.1111.txt");
      let extension = remove_extension(&file_name);
      assert_eq!("test.1111", extension);

      let file_name = String::from("test");
      let extension = remove_extension(&file_name);
      assert_eq!("test", extension);
   }

   #[test]
   fn file_get_file_name_ok() {
      let file_path = String::from("/dummy/dir/test.txt");
      let extension = get_file_name(&file_path);
      assert_eq!("test.txt", extension);
   }

   #[test]
   fn file_read_lines_ok() {
      match read_lines("./Cargo.toml") {
         Some(lines) => {
            let mut lines = lines.enumerate();
            if let Some((l, Ok(str))) = lines.next() {
               assert_eq!(0, l);
               assert_eq!(String::from("[package]"), str);
            } else {
               assert!(false, "file read_line, first line should be [package]");
            }
            if let Some((l, Ok(str))) = lines.next() {
               assert_eq!(1, l);
               assert_eq!(String::from("name = \"oms\""), str);
            } else {
               assert!(false, "file read_line, second line should be name = \"oms\"");
            }
         },
         None => {
            assert!(false, "file read_line should be ok");
         }
      }
   }
}