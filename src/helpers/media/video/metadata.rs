use serde::{Deserialize, Serialize};
use crate::helpers::command;

use super::title::VideoTitle;


#[derive(Debug, Deserialize, Serialize)]
pub struct VideoMetadata {
    pub title: String,
    pub summary: String,
    pub year: u16,
    pub casts: Vec<String>,
    pub genres: Vec<String>,
}

impl VideoMetadata {
    pub fn from(file_path: &String) -> Self {
        let video_title = VideoTitle::from(file_path);

        let mut title = video_title.title.clone();
        let mut summary = file_path.to_string();
        
        let mut date = String::new();
        let mut creation_time = String::new();

        let mut casts = vec![];
        let mut genres = vec![];

        // Get media info from metadata (or exiftool ?)
        let info = command::exec(
            "ffprobe",
             ["-loglevel", "error", "-show_entries", "stream_tags:format_tags", file_path]
        );

        for line in info.lines() {
            if line.trim().is_empty() {
                continue;
            }
            // TAG:title -> title
            // TAG:artist -> casts
            // TAG:comment -> summary
            // TAB:genre -> genres
            // TAG:date -> year
            if line.starts_with("TAG:title") {
                title = line.replace("TAG:title=", "").trim().to_string();
            }
            if line.starts_with("TAG:artist") {
                casts = line.replace("TAG:artist=", "").trim().split(",").map(|c| c.trim().to_string()).collect();
            }
            if line.starts_with("TAG:genre") {
                genres = line.replace("TAG:genre=", "").trim().split(",").map(|c| c.trim().to_string()).collect();
            }
            if line.starts_with("TAG:date") {
                date = line.replace("TAG:date=", "").trim().to_string();
            }
            if line.starts_with("TAG:creation_time") {
                creation_time = line.replace("TAG:creation_time=", "").trim().to_string();
            }
            
            if line.starts_with("TAG:comment") {
                summary = line.replace("TAG:comment=", "").trim().to_string();
            }
        }

        let year = if let Ok(year) = date.get(0..=3).unwrap_or("").parse::<u16>() {
            year
        } else if let Ok(year) = creation_time.get(0..=3).unwrap_or("").parse::<u16>() {
            year
        } else {
            video_title.year.clone()
        };

        return Self {
            title: title,
            summary: summary,
            year: year,
            casts: casts,
            genres: genres,
        };
    }
}
