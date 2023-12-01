use std::thread;
use std::sync::mpsc::Sender;
use crate::helpers::file;
use super::{format_file_display, text_contains, format_line_found};

///
/// 
/// cargo run -- search ./src fn
/// cargo run -- search ./Cargo.toml opt
/// 
pub fn search_in_file(file_path: &String, search_term: &String, tx: Sender<String>) {
    let file_path = file_path.clone();
    let search_term = search_term.to_lowercase().clone();

    thread::spawn(move || {
        let mut result = String::new();

        let mut lines = file::read_lines(&file_path).enumerate();
        let mut line_found = false;
        while let Some((line_number, result_line)) = lines.next() {
            if let Ok(line_text) = result_line {
                if text_contains(&line_text, &search_term) {
                    if line_found == false {
                        result.push_str(&format_file_display(&file_path));
                        line_found = true;
                    }
                    result.push_str(&format_line_found(&line_number.to_string(), &line_text));
                }
            }
        }

        if !result.is_empty() {
            tx.send(result).unwrap_or_default();
        }
    });
}