use core::fmt;
use std::{sync::mpsc::Sender, io};

use colored::Colorize;
use serde::{Deserialize, Serialize};
use sha256::digest;
use crate::helpers::{file, self, db::elastic::Elastic};
use super::option::InfoOption;

pub struct ImageInfo<'a> {
    pub file_path: &'a String,
    pub info_option: &'a InfoOption,
}

impl<'a> ImageInfo<'a> {
    fn get_image_result(&self) -> Result<ImageResult, io::Error> {
        let file_name = file::get_file_name(&self.file_path);
        let relative_file_path = self.file_path.replace(&self.info_option.base_path, "");

        let hash = digest(&relative_file_path);
        Ok(ImageResult {
            title: file_name,
            summary: String::new(),
            file_path: relative_file_path,
            full_path: self.file_path.to_string(),
            hash: hash,
        })    
    }

    pub fn info(&self, tx: Sender<String>) {
        match self.get_image_result() {
            Ok(image) => {
                save_elastic(&image, &self.info_option.elastic);
                tx.send(format!("\
\n------------------------------------------------------------------------
{image}\n")).unwrap_or_default();
            },
            Err(err) => {
                if self.info_option.display_preview == false {
                    println!("\n{}\n", err.to_string().on_red());
                } else {
                    return tx.send(format!("\n{}\n", err.to_string().on_red())).unwrap_or_default();
                }                
            }
        }
    }
}

fn save_elastic(image: &ImageResult, elastic: &Option<Elastic>) {
    if let Some(el) = elastic {
        // Get and save only first result
        el.insert(&image.hash, &image);
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ImageResult {
    pub title: String,
    pub summary: String,

    pub file_path: String,
    pub full_path: String,
    pub hash: String,
}

impl fmt::Display for ImageResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut str = String::new();
        str.push_str(&format!("File name: {}\n\n", self.title.bold()));

        str.push_str(&helpers::output::draw_image(&self.full_path, (50, 50)));
        write!(f, "{str}")
    }
}