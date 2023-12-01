use std::{thread, path::Path};
use std::sync::mpsc::Sender;
use pdf::file::FileOptions;
use super::{format_file_display, text_contains, format_line_found};

///
///
/// https://github.com/pdf-rs/pdf/blob/master/pdf/examples/read.rs
///  
/// cargo run -- search /home/solofo/Documents/books tunique
/// cargo run -- search /home/solofo/Documents/mb_manual_z790-gx-series_e_1201.pdf test
/// 
pub fn search_in_file(file_path: &String, search_term: &String, tx: Sender<String>) {
    let file_path = file_path.clone();
    let search_term = search_term.to_lowercase().clone();

    thread::spawn(move || {
        let mut result = String::new();
        let mut found = vec![];
        let file_name = Path::new(&file_path);
        
        if text_contains(&file_name.file_name().unwrap_or_default().to_str().unwrap_or_default().to_string(), &search_term) {
            found.push(("File", &file_path));
        }

        if let Ok(file) = FileOptions::cached().open(&file_path) {
            if let Some(ref info) = file.trailer.info_dict {
                let title = info.title.as_ref().map(|p| p.to_string_lossy()).unwrap_or_default().to_lowercase();
                let author = info.author.as_ref().map(|p| p.to_string_lossy()).unwrap_or_default().to_lowercase();
                let creator = info.creator.as_ref().map(|p| p.to_string_lossy()).unwrap_or_default().to_lowercase();
                let keywords = info.keywords.as_ref().map(|p| p.to_string_lossy()).unwrap_or_default().to_lowercase();
                let subject = info.subject.as_ref().map(|p| p.to_string_lossy()).unwrap_or_default().to_lowercase();
                
                if text_contains(&title, &search_term) {
                    found.push(("Title", &title));
                }
                if text_contains(&author, &search_term) {
                    found.push(("Author", &author));
                }
                if text_contains(&creator, &search_term) {
                    found.push(("Creator", &creator));
                }
                if text_contains(&keywords, &search_term) {
                    found.push(("Keywords", &keywords));
                }
                if text_contains(&subject, &search_term) {
                    found.push(("Subject", &subject));
                }

                if found.len() > 0 {
                    result.push_str(&format_file_display(&file_path));
                    found.iter().for_each(|(item, text)| {
                        result.push_str(&format_line_found(&item.to_string(), &text));

                    });
                }
                // info.
                // result.push_str(&format!("{file_path}\t-\t{title}\t-\t{author}\t-\t{creator}\t-\t{keywords}\t-\t{subject}\n\n"));
            }
        }
            
        if !result.is_empty() {
            tx.send(result).unwrap_or_default();
        }
    });
}
