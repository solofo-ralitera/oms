use std::ops::Deref;
use regex::Regex;
use crate::helpers::{file, media};

pub struct VideoTitle {
    pub title: String,
    pub year: u16,
    pub language: String,
    pub adult: bool,
}

impl VideoTitle {
    pub fn from(file_path: &String) -> Self {
        let raw_title = file::get_file_name(file_path);
        
        // Remove first [...]
        let re_brakets = Regex::new(r"^\[[^\]]{1,}\]").unwrap();
        let raw_title = re_brakets.replace(&raw_title, "").trim().to_string();
        
        // Get title and Year
        let re_year = Regex::new(r"^(.{1,})[\.\(]([0-9]{4})(.{0,})").unwrap();
        if let Some((_, [title, year, _])) = re_year.captures(&raw_title).map(|c| c.extract()) {
            let title = format_title_remove_point(title);
    
            return VideoTitle { 
                title: media::normalize_media_title(&title), 
                year: year.parse::<u16>().unwrap_or_default(),
                language: "en-US".to_string().clone(),
                adult: false,
            };
        }
    
        let title: String = file::remove_extension(&raw_title);
        let title = format_title_remove_point(&title);
    
        return VideoTitle { 
            title: media::normalize_media_title(&title), 
            year: 0,
            language: String::new(),
            adult: false,
        };
    }

    pub fn normalized(&self) -> String {
        let mut res = String::new();
        res.push_str(&self.title);
        res.push(' ');
        res.push('(');
        res.push_str(&self.year.to_string());
        res.push(')');
        return res;
    }
}

fn format_title_remove_point(title: &str) -> String {
    let re_space = Regex::new(r"([^\.]{2,})(\.)").unwrap();
    let title = re_space.replace_all(&title, "${1} ").deref().to_string();

    let re_point = Regex::new(r"(\.)([^.]{2,})").unwrap();
    let title = re_point.replace_all(&title, "${1} ${2}").deref().to_string();
    return title.trim().to_string();
}
