use std::sync::mpsc::Sender;
use crate::helpers::media::audio;
use super::{option::SearchOption, format_file_display, format_line_found};

pub struct AudioSearch<'a> {
    pub file_path: &'a String,
    pub search_term: &'a String,
    pub search_option: &'a SearchOption,
}

impl<'a> AudioSearch<'a> {
    pub fn search(&self, tx: Sender<String>) {
        let mut result = String::new();
        if let Ok(image) = audio::get_audio_result(&String::new(), self.file_path) {
            let search_results = image.search(self.search_term);
            if search_results.len() > 0 {
                result.push_str(&format_file_display(&self.file_path));
                search_results.iter().for_each(|(item, text)| {
                    result.push_str(&format_line_found(&item.to_string(), &text, &self.search_option));
                });
            }
        }
        if !result.is_empty() {
            tx.send(result).unwrap_or_default();
        }
    }
}