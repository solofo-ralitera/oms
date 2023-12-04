use std::sync::mpsc::Sender;

use super::{format_file_display, text_contains, format_line_found, SearchOption, text_reg_contains};
use crate::helpers::pdf::PdfInfo;
use crate::helpers::file::get_file_name;


///
///
/// https://github.com/pdf-rs/pdf/blob/master/pdf/examples/read.rs
///  
/// cargo run -- search /home/solofo/Documents/books tunique
/// cargo run -- search /home/solofo/Documents/mb_manual_z790-gx-series_e_1201.pdf test
/// 
/// ## Features
/// 
/// * [x] Search in metadata (title, author, keywords...)
/// * [ ] Search in content
/// * [ ] Search in summary?
pub struct PdfSearch<'a> {
    pub file_path: &'a String,
    pub search_term: &'a String,
    pub search_option: &'a SearchOption,
}

impl<'a> PdfSearch<'a> {
    pub fn search(&self, tx: Sender<String>) {
        let mut result = String::new();
        let mut found: Vec<(String, String)> = vec![];
        let file_name = get_file_name(&self.file_path).to_lowercase();

        if text_contains(&file_name, &self.search_term) {
            found.push(("File".to_string(), file_name.clone()));
        }

        if let Ok(pdf_info) = PdfInfo::read(&self.file_path) {
            if text_contains(&pdf_info.title, &self.search_term) {
                found.push(("Title".to_string(), pdf_info.title));
            }
            if text_contains(&pdf_info.author, &self.search_term) {
                found.push(("Author".to_string(), pdf_info.author));
            }
            if text_contains(&pdf_info.creator, &self.search_term) {
                found.push(("Creator".to_string(), pdf_info.creator));
            }
            if text_contains(&pdf_info.keywords, &self.search_term) {
                found.push(("Keywords".to_string(), pdf_info.keywords));
            }
            if text_contains(&pdf_info.subject, &self.search_term) {
                found.push(("Subject".to_string(), pdf_info.subject));
            }

            for (page, content) in pdf_info.content.enumerate() {
                if self.skip_file(&found) {
                    break;
                }
                match text_reg_contains(&content, &self.search_term) {
                    None => (),
                    Some(results) => {
                        for line in results {
                            if self.skip_file(&found) {
                                break;
                            }
                            let text_page = "Page ".to_string() + &page.to_string();
                            found.push((text_page, line));
                        }
                    }
                }
            }

            if found.len() > 0 {
                result.push_str(&format_file_display(&self.file_path));
                found.iter().for_each(|(item, text)| {
                    result.push_str(&format_line_found(&item.to_string(), &text, &self.search_option));

                });
            }
        }

        if !result.is_empty() {
            tx.send(result).unwrap_or_default();
        }
    }    

    fn skip_file(&self, found: &Vec<(String, String)>) -> bool {
        self.search_option.display == "file-only" && found.len() > 0
    }
}
