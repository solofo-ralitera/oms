use std::io;

use serde::{Deserialize, Serialize};
use crate::helpers::command;

type Result<T> = std::result::Result<T, std::io::Error>;


#[derive(Debug, Deserialize, Serialize)]
pub struct PdfMetadata {
    pub title: String, 
    pub summary: String, // Subjet + Description
    pub year: u16, // Date
    pub casts: Vec<String>, // author, split ; or , then remove single or empty char
    pub genres: Vec<String>, // Keywords, split ; or , then remove single or empty char
}

impl PdfMetadata {
    /*
    title => Title
    summary => Description
    year => Date
    casts => Author
    genres => Keywords
    */
    pub fn write(&self, file_path: &String) -> Result<bool> {
        let res = command::exec("exiftool", [
            &format!("-overwrite_original"),
            &format!("-Title={}", self.title),
            &format!("-Date={}", self.year),
            // &format!("-Subject={}", subject),
            &format!("-Description={}", self.summary),
            &format!("-Author={}", self.casts.join(",")),
            &format!("-Keywords={}", self.genres.join(",")),
            &file_path
        ]);
        if res.contains("updated") {
            return Ok(true);
        }

        return Err(io::Error::new(
            io::ErrorKind::Interrupted, 
            format!("Update metadata: file not updated")
        ));
    }

    pub fn write_from_body_content(file_path: &String, body_content: &String) -> Result<bool> {
        match serde_json::from_str::<PdfMetadata>(body_content) {
            Ok(pdf_metadata) => pdf_metadata.write(file_path),
            Err(err) => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput, 
                    format!("Update pdf metadata: invalid json {}", err.to_string())
                ));
            }
        }
    }
}