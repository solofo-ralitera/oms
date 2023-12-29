mod summary;

use crate::{helpers::{file, movie, rtrim_char, input::get_range_params, string}, app::commands::{info::Info, Runnable}};
use once_cell::sync::Lazy;
use regex::Regex;
use urlencoding::decode;
use std::{cmp::min, thread, collections::HashMap};
use super::option::MservOption;

pub struct ProcessParam<'a> {
    pub path: &'a str,
    pub verb: &'a str,
    pub request_header: &'a Vec<String>,
    pub serv_option: &'a MservOption,
}

static STATIC_RESOURCES: Lazy<HashMap<&str, (&str, &[u8])>> = Lazy::new(|| {
    let mut static_resources: HashMap<&str, (&str, &[u8])> = HashMap::new();
    static_resources.insert("/", ("text/html; charset=utf-8", include_bytes!("./resources/assets/index.html")));
    static_resources.insert("/favicon.ico", ("image/x-icon", include_bytes!("./resources/assets/favicon.ico")));
    static_resources.insert("/assets/js/main.js", ("text/javascript", include_bytes!("./resources/assets/js/main.js")));
    
    static_resources.insert("/assets/js/components/movie.js", ("text/javascript", include_bytes!("./resources/assets/js/components/movie.js")));
    static_resources.insert("/assets/js/components/movies.js", ("text/javascript", include_bytes!("./resources/assets/js/components/movies.js")));
    static_resources.insert("/assets/js/components/player.js", ("text/javascript", include_bytes!("./resources/assets/js/components/player.js")));
    static_resources.insert("/assets/js/components/search.js", ("text/javascript", include_bytes!("./resources/assets/js/components/search.js")));
    static_resources.insert("/assets/js/components/summary.js", ("text/javascript", include_bytes!("./resources/assets/js/components/summary.js")));

    static_resources.insert("/assets/js/components/config.js", ("text/javascript", include_bytes!("./resources/assets/js/components/config.js")));
    static_resources.insert("/assets/js/components/config/scandir.js", ("text/javascript", include_bytes!("./resources/assets/js/components/config/scandir.js")));
    static_resources.insert("/assets/js/components/config/summary.js", ("text/javascript", include_bytes!("./resources/assets/js/components/config/summary.js")));

    static_resources.insert("/assets/js/services/app.js", ("text/javascript", include_bytes!("./resources/assets/js/services/app.js")));
    static_resources.insert("/assets/js/services/elastic.js", ("text/javascript", include_bytes!("./resources/assets/js/services/elastic.js")));
    static_resources.insert("/assets/js/services/EventBus.js", ("text/javascript", include_bytes!("./resources/assets/js/services/EventBus.js")));

    return static_resources;
});

///
/// Return: status: 200 OK, headers, content
pub fn process(ProcessParam {path, verb, request_header, serv_option}: ProcessParam) -> (String, Vec<(String, String)>, Option<Box<dyn Iterator<Item = String>>>, Option<Vec<u8>>) {
    if verb == "OPTIONS" {
        return (String::new(), vec![], None, None);
    }
    // Static files
    match STATIC_RESOURCES.get(path) {
        None => (),
        Some((content_type, content)) => {
            if path.ends_with("elastic.js") {
                match serv_option.elastic.as_ref() {
                    Some(elastic) => return (
                        String::from("200 OK"), 
                        vec![
                            (String::from("Content-type"), content_type.to_string()),
                        ], 
                        None,
                        Some(string::bytes_replace(content, b"\"ELASTIC_URL\"", format!("\"{}\"", elastic.url).as_bytes())),
                    ),
                    _ => (),
                };
            }
            if path.ends_with("summary.js") {
                return (
                    String::from("200 OK"), 
                    vec![
                        (String::from("Content-type"), content_type.to_string()),
                    ], 
                    None,
                    Some(string::bytes_replace(content, b"\"BASE_URL\"", format!("\"{}\"", serv_option.base_path).as_bytes())),
                );
            }
            return (
                String::from("200 OK"), 
                vec![
                    (String::from("Content-type"), content_type.to_string()),
                ], 
                None,
                Some(content.to_vec()),
            );
        }
    }
    // Movie files
    if path.starts_with("/movie/") {
        let file_path = decode(path).unwrap_or_default().replace("/movie/", "/");
        return process_video(&file_path, &request_header, &serv_option);
    }
    // Other processes
    return process_command(path, &request_header, &serv_option);
}

fn process_command(path: &str, _: &Vec<String>, serv_option: &MservOption) -> (String, Vec<(String, String)>, Option<Box<dyn Iterator<Item = String>>>, Option<Vec<u8>>) {
    match path {
        "/scan-dir" => {
            scan_movie_dir(serv_option);
            return (String::from("200 OK"), vec![], None, None);
        },
        "/summary" => {
            let summary = serde_json::to_string(&summary::movies_summary(serv_option)).unwrap_or(String::new());
            return (String::from("200 OK"), vec![], None, Some(summary.as_bytes().to_vec()));
        },
        "/all-files-path" => {
            let mut files: Vec<String> = vec![];
            let _ = file::scan(&serv_option.base_path, &mut files);
            let files = serde_json::to_string(&files).unwrap_or(String::new());
            return (String::from("200 OK"), vec![], None, Some(files.as_bytes().to_vec()));
        },
        _ => {
            return (String::from("404 Not Found"), vec![], None, None);
        }
    }
}

fn scan_movie_dir(serv_option: &MservOption) {
    let file_path = serv_option.base_path.to_string();
    if file_path.is_empty() {
        return;
    }
    let mut option = HashMap::new();
    option.insert(String::from("hide-preview"), String::new());
    match serv_option.elastic.as_ref() {
        Some(elastic) => {
            option.insert(String::from("elastic-dsn"), elastic.url.to_string());
        },
        _ => (),
    }
    let info = Info {
        file_path: file_path.to_string(),
        cmd_options: option,
    };
    thread::spawn(move || info.run());
}

fn process_video(file_path: &String, request_header: &Vec<String>, serv_option: &MservOption) -> (String, Vec<(String, String)>, Option<Box<dyn Iterator<Item = String>>>, Option<Vec<u8>>) {
    // serv_option.base_path;
    let file_path = &get_file(&serv_option.base_path, file_path);
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
