use std::{fs, os::unix::fs::MetadataExt};

use serde::{Deserialize, Serialize};
use crate::helpers::{command, string, file};

use super::{title::VideoTitle, result::VideoResult};


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
            // TAG:genre -> genres
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

    pub fn compare_to_videoresult(&self, result: &VideoResult) -> bool {
        if string::compare_normalize(&self.title, &result.title) &&
        string::compare_normalize(&self.summary, &result.summary) &&
        self.year == result.year
        {
            return true;
        }
        return false;
    }

    pub fn write_from_result(file_path: &String, result: VideoResult) -> bool {
        // TAG:title -> title
        // TAG:artist -> casts
        // TAG:comment -> summary
        // TAG:genre -> genres
        // TAG:date -> year
        let current_metadata = Self::from(file_path);
        if current_metadata.compare_to_videoresult(&result) {
            return false;
        }
        let extension = file::get_extension(file_path);
        let file_size = file::file_size(file_path).unwrap_or(1);
        let output_file = format!("{file_path}.oms_metadata_updated.{extension}");
        command::exec("ffmpeg", [
            "-i", file_path,
            "-metadata", &format!("title={}", result.title),
            "-metadata", &format!("artist={}", result.casts.join(", ")),
            "-metadata", &format!("comment={}", result.summary),
            "-metadata", &format!("genre={}", result.genres.join(", ")),
            "-metadata", &format!("date={}", result.year),
            "-codec", "copy",
            "-y", &output_file
        ]);
        match fs::metadata(&output_file) {
            Ok(m) if m.is_file() => {
                let delta = m.size() as f64 / file_size as f64;
                if delta < 1.1 {
                    if let Ok(_) = fs::rename(output_file, file_path) {
                        return true;
                    }
                }
            },
            _ => (),
        };
        return false;
    }
}
