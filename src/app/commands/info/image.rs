use std::sync::mpsc::Sender;
use colored::Colorize;

use crate::helpers::{db::elastic::Elastic, image::{ImageResult, get_image_result}};
use super::option::InfoOption;

pub struct ImageInfo<'a> {
    pub file_path: &'a String,
    pub info_option: &'a InfoOption,
}

impl<'a> ImageInfo<'a> {
    pub fn info(&self, tx: Sender<String>) {
        match get_image_result(&self.info_option.base_path, &self.file_path) {
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
        el.insert(&image.hash, &image);
    }
}
