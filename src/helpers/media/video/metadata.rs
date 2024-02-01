use std::{fs, io};
use regex::Regex;
use serde::{Deserialize, Serialize};
use crate::helpers::{command, string, file};
use super::{title::VideoTitle, result::VideoResult};

type Result<T> = std::result::Result<T, std::io::Error>;

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
        let mut summary = String::new();
        let mut date = String::new();
        let mut creation_time = String::new();
        let mut casts = vec![];
        let mut genres = vec![];

        // Get media info from metadata (or exiftool ?) use -print_format json
        // ffprobe -loglevel error -show_entries stream_tags:format_tags input
        let info = command::exec(
            "ffprobe",
             ["-loglevel", "error", "-show_entries", "stream_tags:format_tags", file_path]
        );

        // Get text between [FORMAT]...[/FORMAT]
        let re_format = Regex::new(r"(?is)\[format\](.{1,})\[/format\]").unwrap();
        let info = if let Some((_, [format])) = re_format.captures(&info).map(|c| c.extract()) {
            format.trim().to_string()
        } else {
            String::new()
        };

        let re_tag = Regex::new(r"^(?i)tag:[a-z_]{1,}=").unwrap();
        let mut current_key = "";
        let mut current_value = String::new();

        for line in info.lines() {
            let value = re_tag.replace(line, "").trim().to_string();
            let l_line = line.to_lowercase();

            current_value = if l_line.starts_with("tag:") {
                format!("{value}")
            } else {
                format!("{current_value}\n{value}")
            };

            current_key = if l_line.starts_with("tag:title") {
                "title"
            } else if l_line.starts_with("tag:artist") {
                "artist"
            } else if l_line.starts_with("tag:genre") {
                "genre"
            } else if l_line.starts_with("tag:date") {
                "date"
            } else if l_line.starts_with("tag:creation_time") {
                "creation_time"
            } else if l_line.starts_with("tag:comment") {
                "comment"
            } else if l_line.starts_with("tag:"){
                "other"
            } else {
                current_key
            };

            // TAG:title -> title
            // TAG:artist -> casts
            // TAG:comment -> summary
            // TAG:genre -> genres
            // TAG:date -> year
            match current_key {
                "title" => title = current_value.trim().to_string(),
                "artist" => casts = current_value.trim().split(",").map(|c| c.trim().to_string()).collect(),
                "genre" => genres = current_value.trim().split(",").map(|c| c.trim().to_string()).collect(),
                "date" => date = current_value.trim().to_string(),
                "creation_time" => creation_time = current_value.trim().to_string(),
                "comment" => summary = current_value.trim().to_string(),
                _ => (),
            };
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

    pub fn eq_videoresult(&self, result: &VideoResult) -> bool {
        if string::compare_normalize(&self.title, &result.title) &&
        string::compare_normalize(&self.summary, &result.summary) &&
        self.year == result.year
        {
            return true;
        }
        return false;
    }

    pub fn write(&self, file_path: &String) -> Result<bool> {
        let extension = file::get_extension(file_path);
        let file_size = file::file_size(file_path).unwrap_or(1);
        let output_file = format!("{file_path}.oms_metadata_updated.{extension}");
        if let Ok(_) = fs::metadata(&output_file) {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists, 
                format!("Update metadata: error: Temp file {} exists", output_file)
            ));
        }

        command::exec("ffmpeg", [
            "-i", file_path,
            "-metadata", &format!("title={}", self.title),
            "-metadata", &format!("artist={}", self.casts.join(", ")),
            "-metadata", &format!("comment={}", self.summary),
            "-metadata", &format!("genre={}", self.genres.join(", ")),
            "-metadata", &format!("date={}", self.year),
            "-codec", "copy",
            "-y", &output_file
        ]);
        if let Ok(m) = fs::metadata(&output_file) {
            let delta = m.len() as f64 / file_size as f64;
            if m.is_file() && delta > 0.95 {
                match fs::rename(output_file, file_path) {
                    Ok(_) => return Ok(true),
                    Err(err) => {
                        return Err(io::Error::new(
                            io::ErrorKind::PermissionDenied, 
                            format!("Metadata write error: {}", err.to_string())
                        ));
                    }
                }
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData, 
                    format!("Metadata write error: insufficient delta {delta}")
                ));
            }
        } else {
            return Err(io::Error::new(
                io::ErrorKind::Interrupted, 
                format!("Metadata write error: ouput {output_file} not created")
            ));
        }
    }

    pub fn write_from_result(file_path: &String, result: VideoResult) -> Result<bool> {
        let current_metadata = Self::from(file_path);
        if current_metadata.eq_videoresult(&result) {
            return Ok(false);
        }
        return VideoMetadata {
            title: result.title,
            summary: result.summary,
            year: result.year,
            casts: result.casts,
            genres: result.genres,
        }.write(file_path);
    }

    pub fn write_from_body_content(file_path: &String, body_content: &String) -> Result<bool> {
        match serde_json::from_str::<VideoMetadata>(body_content) {
            Ok(video_metadata) => return video_metadata.write(file_path),
            Err(err) => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput, 
                    format!("Update video metadata: invalid json {}", err.to_string())
                ));
            }
        }
    }
}
