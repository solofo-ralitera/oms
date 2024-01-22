use serde::{Deserialize, Serialize};
use crate::helpers::command;


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
    summary => 
        Subject
        Description
    year => Date
    casts => Author
    genres => Keywords
    */
    pub fn write(&self, file_path: &String) -> bool {
        let (subject, description) = self.summary.split_once("\n\n").unwrap_or(("", ""));
        let res = command::exec("exiftool", [
            &format!("-Title={}", self.title),
            &format!("-Date={}", self.year),
            &format!("-Subject={}", subject),
            &format!("-Description={}", description),
            &format!("-Author={}", self.casts.join(",")),
            &format!("-Keywords={}", self.genres.join(",")),
            &file_path
        ]);
        return res.contains("updated");
    }

    pub fn write_from_body_content(file_path: &String, body_content: &String) -> bool {
        if let Ok(pdf_metadata) = serde_json::from_str::<PdfMetadata>(body_content) {
            return pdf_metadata.write(file_path);
        }
        return false;
    }
}