use core::fmt;

use colored::Colorize;
use serde::{Deserialize, Serialize};

use crate::helpers::string;


#[derive(Debug, Deserialize, Serialize)]
pub struct PdfResult {
    pub title: String,
    pub summary: String,

    pub year: u16,  
    pub genres: Vec<String>, 
    pub casts: Vec<String>, 

    pub provider: String,

    pub rating: f32,
    pub file_type: String,
    pub file_path: String,
    pub full_path: String,
    pub hash: String,
    pub modification_time: u64,    
    pub duration: usize,
    pub file_size: usize,
}

impl fmt::Display for PdfResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut str = String::new();
        str.push_str(&format!("Title: {} ({})\n", self.title.bold(), self.year));

        // TODO: draw thumb
        // str.push_str(&helpers::output::draw_image(&self.thumb, (50, 50)));
        
        str.push_str(&format!("\n{}\n", self.summary));
        str.push_str(&format!("\nAuthors: {}\n", self.casts.join(", ")));
        str.push_str(&format!("\nGenre: {}\n", self.genres.join(", ")));

        write!(f, "{str}")
    }
}

impl PdfResult {
    pub fn search(&self, term: &String) -> Vec<(&str, String)> {
        let mut result = vec![];
        if string::text_contains(&self.full_path, term) {
            result.push(("File", self.full_path.to_string()));
        }
        if string::text_contains(&self.title, term) {
            result.push(("Title", self.title.to_string()));
        }
        if string::text_contains(&self.summary, term) {
            result.push(("Subject", self.summary.to_string()));
        }
        if string::text_contains(&self.summary, term) {
            result.push(("Summary", self.summary.to_string()));
        }
        if string::text_contains(&self.casts.join(", "), term) {
            result.push(("Authors", self.casts.join(", ")));
        }        
        if string::text_contains(&self.genres.join(", "), term) {
            result.push(("Genres", self.genres.join(", ")));
        }

        return result;
    }
}
