use core::fmt;
use std::{io, process::Command};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use sha256::digest;
use super::{file, string::{text_contains, normalize_media_title}, command};


#[derive(Debug, Deserialize, Serialize)]
pub struct AudioResult {
    pub title: String,
    pub summary: String,

    pub thumb_url: String,
    pub poster_url: String,

    pub rating: f32,
    pub file_type: String,
    pub file_path: String,
    pub full_path: String,
    pub hash: String,
    pub modification_time: u64,
    pub duration: usize,
}

impl fmt::Display for AudioResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut str = String::new();
        str.push_str(&format!("File name: {}\n\n", self.title.bold()));

        write!(f, "{str}")
    }
}

impl AudioResult {
    pub fn search(&self, term: &String) -> Vec<(&str, String)> {
        let mut result = vec![];

        if text_contains(&self.full_path, term) {
            result.push(("File", self.full_path.to_string()));
        }
        if text_contains(&self.title, term) {
            result.push(("Title", self.title.to_string()));
        }
        return result;
    }
}

pub fn audio_duration(file_path: &String) -> usize {
    let mut cmd = Command::new("ffprobe");
    cmd.args(["-i", file_path, "-show_entries", "format=duration", "-v", "quiet", "-of", "csv=p=0"]);
    // println!("{:?}", cmd);
    let output = command::exec(&mut cmd);
    return output.parse::<f64>().unwrap_or(0.).ceil() as usize;
}

pub fn get_audio_result(base_path: &String, file_path: &String) -> Result<AudioResult, io::Error> {
    let file_name = file::get_file_name(file_path);
    let relative_file_path = file_path.replace(base_path, "");

    let hash = file::sha256(file_path).unwrap_or(digest(&relative_file_path));
    let file_duration = audio_duration(&file_path);

    Ok(AudioResult {
        title: normalize_media_title(&file_name),
        summary: String::new(),

        thumb_url: String::from("/assets/img/audio.png"),
        poster_url: String::from("/assets/img/audio.png"),
    
        rating: 1.,
        file_type: String::from("audio"),
        file_path: relative_file_path,
        full_path: file_path.to_string(),
        hash: hash,
        modification_time: file::get_creation_time(file_path),
        duration: file_duration,
    })    
}