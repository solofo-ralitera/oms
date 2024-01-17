use std::sync::mpsc::Sender;
use crate::helpers::{video, file, string::text_contains};
use super::{option::SearchOption, format_file_display, format_line_found};

///
/// cargo run -- search --cache-path="/media/solofo/MEDIA/.oms" "/media/solofo/MEDIA/films/" fire
/// 
pub struct VideoSearch<'a> {
    pub file_path: &'a String,
    pub search_term: &'a String,
    pub search_option: &'a SearchOption,   
}

impl<'a> VideoSearch<'a> {
    pub fn search(&self, tx: Sender<String>) {
        let mut result = String::new();
        let file_name = file::get_file_name(&self.file_path).to_lowercase();

        let mut found: Vec<(String, String)> = vec![];

        if text_contains(&file_name, &self.search_term) {
            found.push(("File".to_string(), file_name.clone()));
        }

        let videos = video::result::get_video_result(
            &file::get_file_name(
                &self.file_path), 
                &self.file_path,
                &String::new(),
                &self.search_option.provider
            ).unwrap_or(vec![]);
        if videos.len() == 0 {
            return;
        }

        for video in &videos {
            let search_results = video.search(&self.search_term);
            if search_results.len() == 0 {
                continue;
            }
            for (key, value) in &search_results {
                found.push((key.to_string(), value.to_string()));
            }
        }

        if found.len() > 0 {
            result.push_str(&format_file_display(&self.file_path));
            found.iter().for_each(|(item, text)| {
                result.push_str(&format_line_found(&item.to_string(), &text, &self.search_option));
            });
        }

        if !result.is_empty() {
            tx.send(result).unwrap_or_default();
        }
    }
}