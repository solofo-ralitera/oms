pub mod tmdb;
pub mod omdb;
pub mod local;

use core::fmt;
use std::{ops::Deref, process::Command, io, fs};
use crate::helpers::{self, file::remove_extension};
use colored::Colorize;
use regex::Regex;
use sha256::digest;
use serde::{Deserialize, Serialize};
use self::{tmdb::TMDb, omdb::OMDb, local::{Local, LocalParam}};
use super::{cache, string::{text_contains, normalize_media_title}, file, command};


///
/// cargo run -- info --cache-path="/media/solofo/MEDIA/.oms" "/media/solofo/MEDIA/films/"
/// 
pub struct VideoTitle {
    pub title: String,
    pub year: String,
    pub language: String,
    pub adult: bool,
}

impl VideoTitle {
    pub fn normalized(&self) -> String {
        let mut res = String::new();
        res.push_str(&self.title);
        res.push(' ');
        res.push('(');
        res.push_str(&self.year);
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

pub fn format_title(raw_title: &String) -> VideoTitle {
    let re_year = Regex::new(r"^(.{1,})[\.\(]([0-9]{4})(.{0,})").unwrap();
    if let Some((_, [title, year, _])) = re_year.captures(&raw_title).map(|c| c.extract()) {
        let title = format_title_remove_point(title);

        return VideoTitle { 
            title: normalize_media_title(&title), 
            year: year.to_string(),
            language: "en-US".to_string().clone(),
            adult: false,
        };
    }

    let title: String = remove_extension(raw_title);
    let title = format_title_remove_point(&title);

    return VideoTitle { 
        title: normalize_media_title(&title), 
        year: String::new(),
        language: String::new(),
        adult: false,
    };
}

//  ffmpeg -i input.avi input.webm
pub fn transcode(file_path: &String, dest_path: Option<&String>, output: &String) -> Result<Option<String>, io::Error> {
    if !file::is_video_file(file_path) {
        return Err(io::Error::new(
            io::ErrorKind::WriteZero, 
            format!("Video transcode: unsupported extension for {file_path}")
        ))
    }
    if !file::VIDEO_EXTENSIONS.contains(&output.as_str()) {
        return Err(io::Error::new(
            io::ErrorKind::WriteZero, 
            format!("Video transcode: unknown extension {output}")
        ))
    }

    let dest_path = match dest_path {
        None => {
            let re = Regex::new(r"(?i)\.[0-9a-z]{2,}$").unwrap();
            re.replace(file_path.as_str(), format!(".{output}")).to_string()
        },
        Some(d) => d.to_string(),
    };

    if file_path.eq(&dest_path) {
        return Ok(None);
    }
    
    if let Ok(_) = file::check_file(&dest_path) {
        return Ok(None);
    }

    println!("Transcoding start {file_path}");
    
    let mut cmd = Command::new("ffmpeg");
    cmd.args(["-i", file_path, &dest_path]);
    command::exec(&mut cmd);

    return match fs::metadata(&dest_path) {
        Ok(metadata) if metadata.is_file() && metadata.len() > 0 => {
            Ok(Some(dest_path))
        },
        _ => Err(io::Error::new(
            io::ErrorKind::WriteZero, 
            format!("Video transcode: {dest_path} not created")
        )),
    };
}

pub fn video_duration(file_path: &String) -> usize {
    if !file::is_video_file(file_path) {
        return 0;
    }
    
    let mut cmd = Command::new("ffprobe");
    cmd.args(["-v", "error", "-show_entries", "format=duration", "-of", "default=noprint_wrappers=1:nokey=1", file_path]);

    let output = command::exec(&mut cmd);
    let mut size = output.parse::<f64>().unwrap_or(0.).ceil() as usize;
    if size == 0 {
        // Try stream option if the first one failed
        let mut cmd = Command::new("ffprobe");
        cmd.args(["-v", "error", "-select_streams", "v:0", "-show_entries", "stream=duration", "-of", "default=noprint_wrappers=1:nokey=1", file_path]);
        let output = command::exec(&mut cmd);
        size = output.parse::<f64>().unwrap_or(0.).ceil() as usize;
    }
    return size;
}


/// Search if file with .mp4 extension existe in the same directory,
/// if found use this mp4 file for next process
/// TODO: live re-encoding for other format than mp4 or ts
/// https://www.reddit.com/r/rust/comments/iplph5/encoding_decoding_video_streams_in_rust/
pub fn get_video_file(file_path: &String) -> String {
    /*
    if !file_path.ends_with(".mp4") {
        let re = Regex::new(r"(?i)\.[0-9a-z]{2,}$").unwrap();
        let mp4_file_path = re.replace(file_path.as_str(), ".mp4").to_string();
        if let Ok(f) = file::check_file(&mp4_file_path) {
            return f.to_string();
        }
    }
    */
    return file_path.clone();
}

/// size: in format width:height, e.g. 600:300, 300:-1 (-1 to keep ratio)
/// at: pick image at x% of video duration, and resize to size
pub fn generate_thumb(src_path: &String, dest_path: &String, size: &str, at: f32) -> Vec<u8> {
    let src_path = get_video_file(src_path);

    // Format duration (s) to hh:mm:ss, :0>2 to keep the leading 0
    let duration = (video_duration(&src_path) as f32 * at).ceil() as usize;
    let duration = format!("{:0>2}:{:0>2}:{:0>2}", (duration / 60) / 60, (duration / 60) % 60, duration % 60);

    // ffmpeg need extenstion in output
    let dest_with_extension = format!("{dest_path}.jpeg");
    let mut cmd = Command::new("ffmpeg");
    cmd.args(["-ss", &duration, "-i", &src_path, "-vf", &format!("scale={size}"), "-frames:v", "1", &dest_with_extension]);
    
    // println!("\n\n{:?}\n\n", cmd);

    command::exec(&mut cmd);

    return match fs::read(&dest_with_extension) {
        Ok(content) => {
            // Remove output extension in final cache file
            let _ = fs::rename(dest_with_extension, dest_path);
            content
        },
        Err(_) => b"".to_vec(),
    };
}


#[derive(Debug, Deserialize, Serialize)]
pub struct VideoResult {
    pub title: String,
    pub summary: String,
    pub year: String,
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
    let video_title = format_title(raw_title);
    let video_hash = digest(video_title.normalized());

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
                print!("Unable to find information about the video: {}, fallback to local provider\n\n", file_path.on_red());
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
    }
    if !base_path.is_empty() {
        cache::write_cache_json(&video_hash, &result, ".video");
    }
    return Ok(result);
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn format_title_1() {
        let content = String::from("10.AAAAA.BBBBB.1111.CC");
        let format_title = format_title(&content);

        assert_eq!("10 AAAAA BBBBB", format_title.title);
        assert_eq!("1111", format_title.year);
        assert_eq!("en-US", format_title.language);
    }

    #[test]
    fn format_title_2() {
        let content = String::from("A.B.C.D.EEEE.1111.XXXXXX");
        let format_title = format_title(&content);

        assert_eq!("A.B.C.D. EEEE", format_title.title);
        assert_eq!("1111", format_title.year);
        assert_eq!("en-US", format_title.language);
    }

    #[test]
    fn format_title_3() {
        let content_0 = String::from("Aaa.Bbbbbbbb.1.1111.TTTTT.eee");
        let format_title_0 = format_title(&content_0);

        assert_eq!("Aaa Bbbbbbbb 1", format_title_0.title);
        assert_eq!("1111", format_title_0.year);
        assert_eq!("en-US", format_title_0.language);

        let content_1 = String::from("Aaa.Bbbbbbbb.1.1111.TTTTT");
        let format_title_1 = format_title(&content_1);

        assert_eq!("Aaa Bbbbbbbb 1", format_title_1.title);
        assert_eq!("1111", format_title_1.year);
        assert_eq!("en-US", format_title_1.language);
    }

    #[test]
    fn format_title_4() {
        let content_0 = String::from("Aaa.Bbbbbbbb.1.Cccccc.ddd (1111).eee");
        let format_title_0 = format_title(&content_0);

        assert_eq!("Aaa Bbbbbbbb 1. Cccccc ddd", format_title_0.title);
        assert_eq!("1111", format_title_0.year);
        assert_eq!("en-US", format_title_0.language);

        let content_1 = String::from("Aaa.Bbbbbbbb.1.Cccccc.ddd (1111)");
        let format_title_1 = format_title(&content_1);

        assert_eq!("Aaa Bbbbbbbb 1. Cccccc ddd", format_title_1.title);
        assert_eq!("1111", format_title_1.year);
        assert_eq!("en-US", format_title_1.language);
    }

    #[test]
    fn format_title_5() {
        let content = String::from("aaa zzzz ee rrrrrrr.AAA");
        let format_title = format_title(&content);

        assert_eq!("aaa zzzz ee rrrrrrr", format_title.title);
        assert!(format_title.year.is_empty());
        assert!(format_title.language.is_empty());
    }

    #[test]
    fn format_title_6() {
        let content_0 = String::from("00 000 AA.bbb");
        let format_title_0 = format_title(&content_0);

        assert_eq!("00 000 AA", format_title_0.title);
        assert!(format_title_0.year.is_empty());
        assert!(format_title_0.language.is_empty());

        let content_1 = String::from("00 000 AA");
        let format_title_1 = format_title(&content_1);

        assert_eq!("00 000 AA", format_title_1.title);
        assert!(format_title_1.year.is_empty());
        assert!(format_title_1.language.is_empty());
    }

    #[test]
    fn format_title_7() {
        let content_0 = String::from("12.3456.avi");
        let format_title_0 = format_title(&content_0);

        assert_eq!("12", format_title_0.title);
        assert_eq!("3456", format_title_0.year);
        assert_eq!("en-US", format_title_0.language);

        let content_1 = String::from("12.3456");
        let format_title_1 = format_title(&content_1);

        assert_eq!("12", format_title_1.title);
        assert_eq!("3456", format_title_1.year);
        assert_eq!("en-US", format_title_1.language);
    }

    #[test]
    fn format_title_8() {
        let content_0 = String::from("1234 (5678).aaa");
        let format_title_0 = format_title(&content_0);

        assert_eq!("1234", format_title_0.title);
        assert_eq!("5678", format_title_0.year);
        assert_eq!("en-US", format_title_0.language);

        let content_0 = String::from("1234 (5678)");
        let format_title_0 = format_title(&content_0);

        assert_eq!("1234", format_title_0.title);
        assert_eq!("5678", format_title_0.year);
        assert_eq!("en-US", format_title_0.language);
    }
    
    #[test]
    fn format_title_9() {
        let content_0 = String::from("Azerty 1234.z");
        let format_title_0 = format_title(&content_0);

        assert_eq!("Azerty 1234", format_title_0.title);
        assert!(format_title_0.year.is_empty());
        assert!(format_title_0.language.is_empty());

        let content_1 = String::from("Azerty 1234");
        let format_title_1 = format_title(&content_1);

        assert_eq!("Azerty 1234", format_title_0.title);
        assert!(format_title_1.year.is_empty());
        assert!(format_title_1.language.is_empty());
    }
    
}
