use core::fmt;
use std::io;

use colored::Colorize;
use serde::{Deserialize, Serialize};
use sha256::digest;

use crate::helpers::output::draw_image;

use super::file;


#[derive(Debug, Deserialize, Serialize)]
pub struct ImageResult {
    pub title: String,
    pub summary: String,

    pub file_type: String,
    pub file_path: String,
    pub full_path: String,
    pub hash: String,
    pub modification_time: u64,
}

impl fmt::Display for ImageResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut str = String::new();
        str.push_str(&format!("File name: {}\n\n", self.title.bold()));

        str.push_str(&draw_image(&self.full_path, (50, 50)));
        write!(f, "{str}")
    }
}

pub fn get_image_result(base_path: &String, file_path: &String) -> Result<ImageResult, io::Error> {
    let file_name = file::get_file_name(file_path);
    let relative_file_path = file_path.replace(base_path, "");

    let hash = file::sha256(file_path).unwrap_or(digest(&relative_file_path));
    Ok(ImageResult {
        title: file_name,
        summary: String::new(),
        file_type: String::from("image"),
        file_path: relative_file_path,
        full_path: file_path.to_string(),
        hash: hash,
        modification_time: file::get_creation_time(file_path),
    })    
}