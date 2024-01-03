use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Read, Write, Seek, SeekFrom};
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::UNIX_EPOCH;
use bytes::Bytes;
use image::EncodableLayout;
use ring::digest::{Context, SHA256};
use data_encoding::HEXUPPER;


type Result<T> = std::result::Result<T, std::io::Error>;

pub static VIDEO_EXTENSIONS: [&str; 24] = ["mpe", "mpv", "m2v", "m4v", "3gp", "3g2", "mp4", "mkv", "avi", "flv", "f4v", "f4p", "f4a", "f4b", "mpg", "mpeg", "mp2", "divx", "wmv", "dat", "webm", "vob", "ogv", "m4p"];
pub static VIDEO_EXTENSIONS_IGNORED: [&str; 9] = ["db", "srt", "nfo", "idx", "sub", "bup", "ifo", "vob", "sfv"];
pub static PDF_EXTENSIONS: [&str; 1] = ["pdf"];
pub static MS_EXTENSIONS: [&str; 6] = ["doc", "docx", "odp", "odt", "pptx", "xlsx"];
pub static IMAGE_EXTENSIONS: [&str; 10] = ["avif", "apng", "gif", "jpg", "jpeg", "jfif", "pjpeg", "pjp", "png", "webp"];

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
      Ok(m) if m.is_file() => Ok(file_path),
      Err(err) => Err(err),
      _ => Err(io::Error::new(
         io::ErrorKind::WriteZero, 
         format!("{file_path} is not a file")
      ))
   }
}

pub fn check_dir(dir_path: &str) -> Result<&str> {
   match fs::metadata(dir_path) {
      Ok(m) if m.is_dir() => Ok(dir_path),
      Err(err) => Err(err),
      _ => Err(io::Error::new(
         io::ErrorKind::WriteZero, 
         format!("{dir_path} is not a directory")
      ))
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

pub fn get_creation_time(file_path: &str) -> u64 {
   if let Ok(metadata) = fs::metadata(file_path) {
      if let Ok(time) = metadata.modified() {
         if let Ok(d) = time.duration_since(UNIX_EPOCH) {
            return d.as_secs();
         }
      }
   }
   return 0;
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

/// Scan the target path and count all the files.
/// https://github.com/yinguobing/count-files/blob/master/src/lib.rs
pub fn scan(
   path: &String,
   record: &mut Vec<String>,
) -> Result<()> {
   // Loop the entries.
   let entries = fs::read_dir(path)?;
   for entry in entries {
       let entry = entry?;
       let path = entry.path();

       // The entry is a directory or a file?
       if path.is_dir() {
           let _ = scan(&path.as_path().to_string_lossy().to_string(), record);
       } else if let Ok(cpath) = fs::canonicalize(&path) {
           record.push(cpath.to_string_lossy().to_string());
       }
   }
   Ok(())
}

// Scan the target path and count all the files.
pub fn scan_count_by_extension(
   path: &String,
   record: &mut HashMap<String, usize>,
) -> Result<()> {
   let entries = fs::read_dir(path)?;
   for entry in entries {
       let entry = entry?;
       let path = entry.path();

       // The entry is a directory or a file?
       if path.is_dir() {
           let _ = scan_count_by_extension(&path.as_path().to_string_lossy().to_string(), record);
       } else if let Some(extension) = path.extension() {
           let extension = extension.to_str().unwrap().to_string();
           let counter = record
               .get(&extension)
               .copied()
               .unwrap_or(0) + 1;
           // Increment extension number
           record.insert(extension.to_string(), counter);
       }
   }
   Ok(())
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