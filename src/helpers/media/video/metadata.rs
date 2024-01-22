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
            else if line.starts_with("TAG:Title") {
                title = line.replace("TAG:Title=", "").trim().to_string();
            }
            else if line.starts_with("TAG:TITLE") {
                title = line.replace("TAG:TITLE=", "").trim().to_string();
            }

            else if line.starts_with("TAG:artist") {
                casts = line.replace("TAG:artist=", "").trim().split(",").map(|c| c.trim().to_string()).collect();
            }
            else if line.starts_with("TAG:Artist") {
                casts = line.replace("TAG:Artist=", "").trim().split(",").map(|c| c.trim().to_string()).collect();
            }
            else if line.starts_with("TAG:ARTIST") {
                casts = line.replace("TAG:ARTIST=", "").trim().split(",").map(|c| c.trim().to_string()).collect();
            }

            else if line.starts_with("TAG:genre") {
                genres = line.replace("TAG:genre=", "").trim().split(",").map(|c| c.trim().to_string()).collect();
            }
            else if line.starts_with("TAG:Genre") {
                genres = line.replace("TAG:Genre=", "").trim().split(",").map(|c| c.trim().to_string()).collect();
            }
            else if line.starts_with("TAG:GENRE") {
                genres = line.replace("TAG:GENRE=", "").trim().split(",").map(|c| c.trim().to_string()).collect();
            }

            else if line.starts_with("TAG:date") {
                date = line.replace("TAG:date=", "").trim().to_string();
            }
            else if line.starts_with("TAG:Date") {
                date = line.replace("TAG:Date=", "").trim().to_string();
            }
            else if line.starts_with("TAG:DATE") {
                date = line.replace("TAG:DATE=", "").trim().to_string();
            }


            else if line.starts_with("TAG:creation_time") {
                creation_time = line.replace("TAG:creation_time=", "").trim().to_string();
            }
            else if line.starts_with("TAG:Creation_time") {
                creation_time = line.replace("TAG:Creation_time=", "").trim().to_string();
            }
            else if line.starts_with("TAG:Creation_Time") {
                creation_time = line.replace("TAG:Creation_Time=", "").trim().to_string();
            }
            else if line.starts_with("TAG:CREATION_TIME") {
                creation_time = line.replace("TAG:CREATION_TIME=", "").trim().to_string();
            }
            
            else if line.starts_with("TAG:comment") {
                summary = line.replace("TAG:comment=", "").trim().to_string();
            }
            else if line.starts_with("TAG:Comment") {
                summary = line.replace("TAG:Comment=", "").trim().to_string();
            }
            else if line.starts_with("TAG:COMMENT") {
                summary = line.replace("TAG:COMMENT=", "").trim().to_string();
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

    pub fn eq_videoresult(&self, result: &VideoResult) -> bool {
        if string::compare_normalize(&self.title, &result.title) &&
        string::compare_normalize(&self.summary, &result.summary) &&
        self.year == result.year
        {
            return true;
        }
        return false;
    }

    pub fn write(&self, file_path: &String) -> bool {
        let extension = file::get_extension(file_path);
        let file_size = file::file_size(file_path).unwrap_or(1);
        let output_file = format!("{file_path}.oms_metadata_updated.{extension}");
        if let Ok(_) = fs::metadata(&output_file) {
            println!("exists");
            return false;
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
            let delta = m.size() as f64 / file_size as f64;
            if m.is_file() && delta > 0.95 {
                match fs::rename(output_file, file_path) {
                    Ok(_) => return true,
                    Err(err) => {
                        println!("Metadata write error: {}", err.to_string());
                        return false;
                    }
                }
            } else {
                println!("Metadata write error: insufficient delta {delta}");
            }
        } else {
            println!("Metadata write error: ouput {output_file} not created");
        }
        return false;
    }

    pub fn write_from_result(file_path: &String, result: VideoResult) -> bool {
        let current_metadata = Self::from(file_path);
        if current_metadata.eq_videoresult(&result) {
            return false;
        }
        return VideoMetadata {
            title: result.title,
            summary: result.summary,
            year: result.year,
            casts: result.casts,
            genres: result.genres,
        }.write(file_path);
    }

    pub fn write_from_body_content(file_path: &String, body_content: &String) -> bool {
        if let Ok(video_metadata) = serde_json::from_str::<VideoMetadata>(body_content) {
            return video_metadata.write(file_path);
        }
        return false;
    }
}
