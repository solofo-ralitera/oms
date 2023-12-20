use crate::helpers::{file, movie, rtrim_char, input::get_range_params};
use regex::Regex;
use urlencoding::decode;
use std::{cmp::min, thread};
use super::option::MservOption;

pub struct ProcessParam<'a> {
    pub path: &'a str,
    pub verb: &'a str,
    pub request_header: &'a Vec<String>,
    pub serv_option: &'a MservOption,
}

///
/// Return: status: 200 OK, headers, content
/// 
// pub fn process(path: &str, verb: &str, request_header: &Vec<String>) -> (String, Vec<(String, String)>, Option<Box<dyn Iterator<Item = String>>>, Option<Vec<u8>>) {
pub fn process(ProcessParam {path, verb, request_header, serv_option}: ProcessParam) -> (String, Vec<(String, String)>, Option<Box<dyn Iterator<Item = String>>>, Option<Vec<u8>>) {
    if verb == "OPTIONS" {
        return (String::new(), vec![], None, None);
    }
    let path = if path == "/" {
        "/assets/index.html"
    } else if path == "/favicon.ico" {
        "/assets/favicon.ico"
    } else {
        path
    };
    let mut file_path = String::new();
    if path.starts_with("/assets/") {
        file_path = format!("./resources/mserv{}", decode(path).unwrap_or_default());
    }
    if path.starts_with("/movie/") {
        file_path = decode(path).unwrap_or_default().replace("/movie/", "/");
    }
    if file_path.is_empty() {
        return (String::from("404 Not Found"), vec![], None, None);
    }
    let mime = file::get_mimetype(path).to_string();
    // Binary content
    if mime.starts_with("image") {
        return (
            String::from("200 OK"), 
            vec![
                (String::from("Content-type"), file::get_mimetype(path).to_string()),
            ], 
            None,
            Some(file::read_buf(&file_path)),
        );
    } else if mime.starts_with("video") {
        // serv_option.base_path;
        let file_path = &get_file(&serv_option.base_path, &file_path);
        let file_size = file::file_size(&file_path).unwrap_or_default();
        let buffer: u64 = 1_500_000;
        
        let (start_range, _) = get_range_params(&request_header, file_size).unwrap_or((0, buffer));
        let end_range = min(start_range + buffer, file_size) - 1;

        let byte_count = end_range - start_range + 1;
        return (
            String::from("206 Partial Content"), 
            vec![
                (String::from("Content-type"), format!("video/{}", file::get_extension(&file_path))), // TODO: fix mime
                (String::from("Accept-Ranges"), String::from("bytes")),
                (String::from("Content-Range"), format!("bytes {start_range}-{end_range}/{file_size}")),
                (String::from("Content-Length"), format!("{}", byte_count)),
            ], 
            None,
            Some(file::read_range(&file_path, start_range, byte_count).unwrap()),
        );
    }
    // Text content
    return (
        String::from("200 OK"), 
        vec![
            (String::from("Content-type"), file::get_mimetype(path).to_string()),
        ], 
        match file::read_lines(&file_path) {
            Some(lines) => Some(Box::new(lines.map(|l| l.unwrap_or_default()))),
            None => None,
        },
        None
    );
}

///
/// TODO: live re-encoding for other format than mp4 or ts
/// https://www.reddit.com/r/rust/comments/iplph5/encoding_decoding_video_streams_in_rust/
fn get_file(base_path: &String, file_path: &String) -> String {
    let file_path = rtrim_char(base_path, '/') + file_path;
    if !file_path.ends_with(".mp4") && !file_path.ends_with(".mkv") && !file_path.ends_with(".ts") {
        let re = Regex::new(r"(?i)\.[a-z]{3}$").unwrap();
        let mp4_file_path = re.replace(file_path.as_str(), ".mp4").to_string();
        match file::check_file(&mp4_file_path) {
            Ok(f) => return f.to_string(),
            Err(_) => {
                // TODO: si avi => re-encode
                let input = file_path.clone();
                let output = mp4_file_path.clone();
                thread::spawn(move || movie::avi_to_mp4(&input, &output));
                return file_path.to_string();
            },
        }
    }
    return file_path.clone();
}
