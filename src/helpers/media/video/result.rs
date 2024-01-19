use serde::{Deserialize, Serialize};
use core::fmt;
use std::io;
use colored::Colorize;
use sha256::digest;

use crate::helpers::{self, string::text_contains, file, cache};

use super::{video_duration, title::VideoTitle, provider::{tmdb::TMDb, omdb::OMDb, local::{Local, LocalParam}}};


#[derive(Debug, Deserialize, Serialize)]
pub struct VideoResult {
    pub title: String,
    pub summary: String,
    pub year: u16,
    pub genres: Vec<String>,
    pub casts: Vec<String>,
    pub thumb_url: String,
    pub thumb: String,
    pub poster_url: String,    
    pub rating: f32,

    pub provider: String,
    pub provider_id: String,

    pub file_path: String,
    pub file_type: String,
    pub hash: String,
    pub modification_time: u64,
    pub duration: usize,
    pub file_size: usize,
}

impl fmt::Display for VideoResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut str = String::new();
        str.push_str(&format!("Title: {} ({})\n\n", self.title.bold(), self.year));

        str.push_str(&helpers::output::draw_image(&self.thumb, (50, 50)));
        str.push_str(&format!("{}\n", self.poster_url));
        
        str.push_str(&format!("\n{}\n", self.summary));
        str.push_str(&format!("\nGenre: {}\n", self.genres.join(", ")));
        str.push_str(&format!("\nCast: {}\n", self.casts.join(", ")));
        write!(f, "{str}")
    }
}

impl VideoResult {
    pub fn search(&self, term: &String) -> Vec<(&str, String)> {
        let mut result = vec![];
        if text_contains(&self.title, term) {
            result.push(("Title", self.title.to_string()));
        }
        if text_contains(&self.summary, term) {
            result.push(("Summary", self.summary.to_string()));
        }
        if text_contains(&self.genres.join(", "), term) {
            result.push(("Genres", self.genres.join(", ")));
        }
        if text_contains(&self.casts.join(", "), term) {
            result.push(("Casts", self.casts.join(", ")));
        }
        return result;
    }
}

pub fn get_video_result(raw_title: &String, file_path: &String, base_path: &String, provider: &String) -> Result<Vec<VideoResult>, io::Error> {
    let video_title = VideoTitle::from(raw_title);
    let file_size = file::file_size(file_path).unwrap_or_default() as usize;
    let video_hash = digest(format!("{}.{file_size}", video_title.normalized()));

    // Warn if year is empty, (omdb and tmdb need year for more accuracy)
    if provider.eq("api") && video_title.year == 0 {
        print!("{}: empty year\n", file_path.yellow());
    }

    // Fist check if result is in cache
    //  If provider in cache is different from supplied provider => force none to reload data
    let mut videos = if let Some((_, content)) = cache::get_cache(&video_hash, ".video") {
        match serde_json::from_str::<Vec<VideoResult>>(&content) {
            Ok(result) if result.len() > 0 => {
                if result.iter().any(|r| r.provider.ne(provider)) {
                    None
                } else {
                    Some(result)
                }
            },
            _ => None,
        }
    } else {
        None
    };

    // Then search in tmdb, if not found switch to omdb
    if videos.is_none() {
        videos = match provider.as_str() {
            "api" => if let Ok(result) = TMDb::info(&video_title) {
                Some(result)
            } else if let Ok(result) = OMDb::info(&video_title) {
                Some(result)
            } else {
                println!(
                    "{}",
                    format!("Unable to find information about the video: {}, fallback to local provider", file_path).yellow()
                );
                None
            },
            _ => None,
        };
    }

    // Lastly, fill result with local data
    if videos.is_none() {
        videos = if let Ok(result) = Local::info(LocalParam {
            video_title: &video_title,
            raw_title: raw_title,
            file_path: file_path,
            base_path: base_path,
        }) {
            Some(result)
        } else {
            None
        }
    }

    if videos.is_none() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput, 
            format!("Unable to find information about the video: {}", file_path.red())
        ));
    }

    let file_time = file::get_creation_time(file_path);
    let file_duration = video_duration(&file_path);

    let mut result = videos.unwrap();
    for video in &mut result {
        video.file_path = file_path.replace(base_path, "");
        video.hash = video_hash.clone();
        video.modification_time = file_time;
        video.duration = file_duration;
        video.file_size = file_size;
    }
    if !base_path.is_empty() {
        // TODO: dis/enable cache
        cache::write_cache_json(&video_hash, &result, ".video");
    }
    return Ok(result);
}
