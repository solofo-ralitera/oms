use std::sync::mpsc::Sender;
use crate::helpers::file::{self, get_file_name};
use super::{format_file_display, text_contains, format_line_found, SearchOption};

///
/// 
/// cargo run -- search ./src fn
/// cargo run -- search ./Cargo.toml opt
/// 
pub struct TextSearch<'a> {
    pub file_path: &'a String,
    pub search_term: &'a String,
    pub search_option: &'a SearchOption,
}

impl<'a> TextSearch<'a> {
    pub fn search(&self, tx: Sender<String>) {
        // TODO :search in file name
        let mut result = String::new();
        let file_name = get_file_name(&self.file_path).to_lowercase();

        let mut line_found = false;

        if text_contains(&file_name, &self.search_term) {
            result.push_str(&format_file_display(&self.file_path));
            line_found = true;
        }

        if let Some(lines) = file::read_lines(&self.file_path) {
            let mut lines = lines.enumerate();
            while let Some((line_number, result_line)) = lines.next() {
                if let Ok(line_text) = result_line {
                    if text_contains(&line_text, &self.search_term) {
                        if line_found == false {
                            result.push_str(&format_file_display(&self.file_path));
                            line_found = true;
                        }
                        result.push_str(&format_line_found(&line_number.to_string(), &line_text, &self.search_option));
                    }
                }
            }
        }

        if !result.is_empty() {
            tx.send(result).unwrap_or_default();
        }
    }    
}
