use regex::Regex;
use crate::helpers::{command, media::pdf::metadata::PdfMetadata, file};


pub fn from_exif(file_path: &String) -> Option<PdfMetadata> {
    let info = command::exec("exiftool",[file_path]);
    if info.trim().is_empty() {
        return None;
    }

    let mut title = String::new();
    // Summary
    let mut description = String::new();
    // Year
    let mut date = String::new();
    // Author
    let mut author = String::new();
    // Genres
    let mut keywords = String::new();

    
    let re_space = Regex::new(" {1,}").unwrap();

    for line in info.lines() {
        let line = re_space.replace(line, " ").to_string();

        if line.starts_with("Author :") {
            author = line.replace("Author :", "").trim().to_string();
        }
        if line.starts_with("Keywords :") {
            keywords = line.replace("Keywords :", "").trim().to_string();
        }
        if line.starts_with("Date :") {
            date = line.replace("Date :", "").trim().to_string();
        }            
        if line.starts_with("Description :") {
            description = line.replace("Description :", "").trim().to_string();
        }
        if line.starts_with("Title :") {
            title = line.replace("Title :", "").trim().to_string();
        }
    }
    
    let year = if let Ok(year) = date.get(0..=3).unwrap_or("").parse::<u16>() {
        year
    } else {
        0
    };

    let re_comma = Regex::new(r"[,;]").unwrap();
    let casts: Vec<String> = re_comma.split(&author).into_iter().map(|l| l.trim().to_string()).filter(|l| l.len() > 1).collect();
    
    let genres: Vec<String>;
    if keywords.contains("/") {
        let re_slash = Regex::new(r"/").unwrap();
        genres = re_slash.split(&keywords).into_iter().map(|l| l.replace(",", "").trim().to_string()).filter(|l| l.len() > 1).collect();
    } else {
        genres = re_comma.split(&keywords).into_iter().map(|l| l.trim().to_string()).filter(|l| l.len() > 1).collect();
    }
    if title.is_empty() {
        title = file::get_file_name(file_path);
    }

    return Some(PdfMetadata {
        title: title,
        summary: description,
        year: year, // Date
        casts: casts,
        genres: genres,
    });
}