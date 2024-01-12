use core::fmt;
use std::{io, fs};

use colored::Colorize;
use pdf::{file::FileOptions, PdfError};
use lopdf::Document;
use serde::{Deserialize, Serialize};
use sha256::digest;
use super::{string::{text_contains, normalize_media_title}, file, command, rtrim_char, ltrim_char};

pub struct PdfInfo {
    pub title: String,
    pub author: String,
    pub creator: String,
    pub keywords: String,
    pub subject: String,
    pub content: PdfContent,
}

impl PdfInfo {
    pub fn read(file_path: &str) -> Result<PdfInfo, PdfError> {
        match FileOptions::cached().open(&file_path) {
            Ok(file) => {

                if let Some(ref info) = file.trailer.info_dict {
                    return Ok(PdfInfo {
                        title: info.title.as_ref().map(|p| p.to_string_lossy()).unwrap_or_default(),
                        author: info.author.as_ref().map(|p| p.to_string_lossy()).unwrap_or_default(),
                        creator: info.creator.as_ref().map(|p| p.to_string_lossy()).unwrap_or_default(),
                        keywords: info.keywords.as_ref().map(|p| p.to_string_lossy()).unwrap_or_default(),
                        subject: info.subject.as_ref().map(|p| p.to_string_lossy()).unwrap_or_default(),
                        content: PdfContent::new(file_path),
                    });
                }
                return Ok(PdfInfo {
                    title: "".to_string(),
                    author: "".to_string(),
                    creator: "".to_string(),
                    keywords: "".to_string(),
                    subject: "".to_string(),
                    content: PdfContent::new(file_path),
                });
            },
            Err(err) => Err(err),
        }
    }
}

pub struct PdfContent {
    current_page: usize,
    page_number: usize,
    document: Document,
}

///
/// https://ahmadrosid.com/blog/extract-text-from-pdf-in-rust
/// 
impl PdfContent {
    fn new(file_path: &str) -> Self {
        if let Ok(document) = Document::load(file_path) {
            return PdfContent {
                current_page: 1,
                page_number: document.get_pages().len(),
                document: document,
            };
        }
        return PdfContent {
            current_page: 1,
            page_number: 0,
            document: Document::new(),
        }
    }
}

impl Iterator for PdfContent {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_page > self.page_number {
            return None;
        }
        let text = self.document.extract_text(&[self.current_page as u32]);
        self.current_page += 1;
        Some(text.unwrap_or_default().replace("?Identity-H Unimplemented?", ""))
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PdfResult {
    pub title: String,
    pub summary: String,
    pub author: String,
    pub creator: String,
    pub keywords: String,
    pub subject: String,

    pub rating: f32,
    pub file_type: String,
    pub file_path: String,
    pub full_path: String,
    pub hash: String,
    pub modification_time: u64,    
    pub duration: usize,
    pub file_size: usize,
}

impl fmt::Display for PdfResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut str = String::new();
        str.push_str(&format!("Title: {}\n", self.title.bold()));
        str.push_str(&format!("Subject: {}\n\n", self.subject));

        // TODO: draw thumb
        // str.push_str(&helpers::output::draw_image(&self.thumb, (50, 50)));
        
        if !self.summary.is_empty() {
            str.push_str(&format!("Summary: {}\n", self.summary));
        }

        if !self.author.is_empty() {
            str.push_str(&format!("Author: {}\n", self.author));
        }
        if !self.keywords.is_empty() {
            str.push_str(&format!("Keywords: {}\n", self.keywords));
        }
        if !self.creator.is_empty() {
            str.push_str(&format!("Creator: {}\n", self.creator));
        }
        write!(f, "{str}")
    }
}

impl PdfResult {
    pub fn search(&self, term: &String) -> Vec<(&str, String)> {
        let mut result = vec![];
        if text_contains(&self.full_path, term) {
            result.push(("File", self.full_path.to_string()));
        }
        if text_contains(&self.title, term) {
            result.push(("Title", self.title.to_string()));
        }
        if text_contains(&self.subject, term) {
            result.push(("Subject", self.subject.to_string()));
        }
        if text_contains(&self.summary, term) {
            result.push(("Summary", self.summary.to_string()));
        }
        if text_contains(&self.author, term) {
            result.push(("Author", self.author.to_string()));
        }
        if text_contains(&self.creator, term) {
            result.push(("Creator", self.creator.to_string()));
        }
        if text_contains(&self.keywords, term) {
            result.push(("Keywords", self.keywords.to_string()));
        }
        return result;
    }
}

pub fn get_pdf_result(base_path: &String, file_path: &String) -> Result<PdfResult, io::Error> {
    let file_size: usize = file::file_size(file_path).unwrap_or_default() as usize;
    let file_name = file::get_file_name(file_path);
    let relative_file_path = file_path.replace(base_path, "");

    let hash = file::sha256(file_path).unwrap_or(digest(&relative_file_path));

    let modification_time = file::get_creation_time(file_path);

    match PdfInfo::read(file_path) {
        Ok(pdf_info) => return Ok(PdfResult {
            title: if pdf_info.title.is_empty() {
                normalize_media_title(&file_name)
            } else {
                normalize_media_title(&pdf_info.title)
            },
            summary: String::new(),
            author: pdf_info.author,
            creator: pdf_info.creator,
            keywords: pdf_info.keywords,
            subject: pdf_info.subject,
            
            rating: 1.,
            file_type: String::from("pdf"),
            file_path: relative_file_path,
            full_path: file_path.to_string(),
            hash: hash,
            modification_time: modification_time,
            duration: 0,
            file_size: file_size,
        }),
        Err(_) => return Ok(PdfResult {
            title: normalize_media_title(&file_name),
            summary: String::new(),
            author: String::new(),
            creator: String::new(),
            keywords: String::new(),
            subject: String::new(),
            
            rating: 1.,
            file_type: String::from("pdf"),
            file_path: relative_file_path,
            full_path: file_path.to_string(),
            hash: hash,
            modification_time: modification_time,
            duration: 0,
            file_size: 0,
        })
    }
}

// TODO: generate thumb
pub fn generate_thumb(src_path: &String, dest_path: &String, size: &str) -> Vec<u8> {
    let size = size.replace(":", "x");
    let size = size.replace("-1", "");
    let size = rtrim_char(&size.to_string(), 'x');
    let size = ltrim_char(&size.to_string(), 'x');

    // Output need extenstion in output
    let dest_with_extension = format!("{dest_path}.jpeg");
    command::exec_stdout(
        "convert",
        ["-thumbnail", &format!("{size}^>"), "-background", "white", "-alpha", "remove", &format!("{src_path}[0]"), &dest_with_extension]    
    );

    return match fs::read(&dest_with_extension) {
        Ok(content) => {
            // Remove output extension in final cache file
            let _ = fs::rename(dest_with_extension, dest_path);
            content
        },
        Err(_) => b"".to_vec(),
    };
}
