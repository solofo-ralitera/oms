use std::{sync::mpsc::Sender};
use dotext::*;
use std::io::Read;
use crate::helpers::file::{get_extension, get_file_name};

use super::{SearchOption, text_contains, format_line_found, format_file_display, text_reg_contains};

/// 
/// cargo run -- search /home/solofo/Downloads FA
/// 
pub struct MsSearch<'a> {
    pub file_path: &'a String,
    pub search_term: &'a String,
    pub search_option: &'a SearchOption,    
}

impl<'a> MsSearch<'a> {
    fn get_file_content(file_path: &String) -> String {
        let mut content = String::new();
        match get_extension(&file_path).to_lowercase().as_str() {
            "docx" => {
                let mut ms_file = Docx::open(file_path).expect("Cannot open file");
                let _ = ms_file.read_to_string(&mut content);
            },
            "xlsx" => {
                let mut ms_file = Xlsx::open(file_path).expect("Cannot open file");
                let _ = ms_file.read_to_string(&mut content);
            },
            "pptx" => {
                let mut ms_file = Pptx::open(file_path).expect("Cannot open file");
                let _ = ms_file.read_to_string(&mut content);
            },
            _ => {
                content.push_str("");
            }
        }
        content
    }

    pub fn search(&self, tx: Sender<String>) {
        let mut found: Vec<(String, String)> = vec![];

        let mut result: String = String::new();
        let file_name = get_file_name(&self.file_path).to_lowercase();

        if text_contains(&file_name, &self.search_term) {
            found.push(("File".to_string(), file_name.clone()));
        }

        let content = Self::get_file_content(&self.file_path);

        match text_reg_contains(&content, &self.search_term) {
            None => (),
            Some(results) => {
                for line in results {
                    if self.skip_file(&found) {
                        break;
                    }
                    found.push(("".to_string(), line.to_string()));
                }
            }
        }

        if found.len() > 0 {
            result.push_str(&format_file_display(&self.file_path));
            found.iter().for_each(|(item, text)| {
                result.push_str(&format_line_found(&item.to_string(), &text, &self.search_option));

            });

            tx.send(result).unwrap_or_default();
        }
    }

    fn skip_file(&self, found: &Vec<(String, String)>) -> bool {
        self.search_option.display == "file-only" && found.len() > 0
    }
}