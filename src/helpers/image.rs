use core::fmt;
use std::io;

use colored::Colorize;
use serde::{Deserialize, Serialize};
use sha256::digest;

use crate::helpers::output::draw_image;

use super::{file, string::{text_contains, normalize_media_title}};


#[derive(Debug, Deserialize, Serialize)]
pub struct ImageResult {
    pub title: String,
    pub summary: String,

    pub rating: f32,
    pub file_type: String,
    pub file_path: String,
    pub full_path: String,
    pub hash: String,
    pub modification_time: u64,
    pub duration: usize,
}

impl fmt::Display for ImageResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut str = String::new();
        str.push_str(&format!("File name: {}\n\n", self.title.bold()));

        str.push_str(&draw_image(&self.full_path, (50, 50)));
        write!(f, "{str}")
    }
}

impl ImageResult {
    pub fn search(&self, term: &String) -> Vec<(&str, String)> {
        let mut result = vec![];

        if text_contains(&self.full_path, term) {
            result.push(("File", self.full_path.to_string()));
        }
        if text_contains(&self.title, term) {
            result.push(("Title", self.title.to_string()));
        }
        if text_contains(&self.summary, term) {
            result.push(("Summary", self.summary.to_string()));
        }
        return result;
    }
}


pub fn get_image_result(base_path: &String, file_path: &String) -> Result<ImageResult, io::Error> {
    let file_name = file::get_file_name(file_path);
    let relative_file_path = file_path.replace(base_path, "");

    let hash = file::sha256(file_path).unwrap_or(digest(&relative_file_path));

    Ok(ImageResult {
        title: normalize_media_title(&file_name),
        summary: String::new(),

        rating: 1.,
        file_type: String::from("image"),
        file_path: relative_file_path,
        full_path: file_path.to_string(),
        hash: hash,
        modification_time: file::get_creation_time(file_path),
        duration: 0,
    })    
}