use std::sync::mpsc::Sender;
use crate::helpers::{command, media::pdf::{get_pdf_result, content::PdfContent}};
use super::{format_file_display, format_line_found, SearchOption, text_reg_contains};


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
    fn loop_content<I>(&self, contents: I, found: &mut Vec<(String, String)>)
    where 
        I: Iterator<Item = String>
    {
        for (page, content) in contents.enumerate() {
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
    }

    pub fn search(&self, tx: Sender<String>) {
        let mut result = String::new();
        let mut found: Vec<(String, String)> = vec![];

        if let Ok(pdf) = get_pdf_result(&String::new(), self.file_path) {
            let search_results = pdf.search(self.search_term);
            if search_results.len() > 0 {
                search_results.iter().for_each(|(item, text)| {
                    found.push((item.to_string(), text.to_string()));
                });
            }
        }
        // pdftotext is slower?
        let content = command::exec("___pdftotext", ["-layout", &self.file_path, "-"]);
        if !content.is_empty() {
            self.loop_content(
                content.lines().map(|l|l.to_string()),
                &mut found
            );
        } else {
            let content = PdfContent::new(&self.file_path);
            self.loop_content(content, &mut found);
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

    fn skip_file(&self, found: &Vec<(String, String)>) -> bool {
        self.search_option.display == "file-only" && found.len() > 0
    }
}
