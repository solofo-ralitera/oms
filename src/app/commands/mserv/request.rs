mod summary;

use crate::{helpers::{file, rtrim_char, input::get_range_params, string, cache, movie, pdf, ltrim_char}, app::commands::{info::Info, Runnable, transcode::Transcode}};
use once_cell::sync::Lazy;
use regex::Regex;
use sha256::digest;
use urlencoding::decode;
use std::{cmp::min, thread, collections::HashMap, fs};
use super::option::MservOption;
use rand::Rng;

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
    static_resources.insert("/assets/js/components/config/genres.js", ("text/javascript", include_bytes!("./resources/assets/js/components/config/genres.js")));

    static_resources.insert("/assets/js/services/app.js", ("text/javascript", include_bytes!("./resources/assets/js/services/app.js")));
    static_resources.insert("/assets/js/services/elastic.js", ("text/javascript", include_bytes!("./resources/assets/js/services/elastic.js")));
    static_resources.insert("/assets/js/services/EventBus.js", ("text/javascript", include_bytes!("./resources/assets/js/services/EventBus.js")));
    static_resources.insert("/assets/js/services/history.js", ("text/javascript", include_bytes!("./resources/assets/js/services/history.js")));

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
                        vec![(String::from("Content-type"), content_type.to_string())], 
                        None,
                        Some(string::bytes_replace(content, b"\"ELASTIC_URL\"", format!("\"{}\"", elastic.url).as_bytes())),
                    ),
                    _ => (),
                };
            }
            if path.ends_with("summary.js") {
                return (
                    String::from("200 OK"), 
                    vec![(String::from("Content-type"), content_type.to_string())], 
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
    // Thumb files (width=300)
    if path.starts_with("/thumb/") {
        let file_path = decode(path).unwrap_or_default().replace("/thumb/", "/");
        return process_thumb(&file_path, &serv_option, "300:-1");
    }
    // Poster files (no resize)
    if path.starts_with("/poster/") {
        let file_path = decode(path).unwrap_or_default().replace("/poster/", "/");
        return process_thumb(&file_path, &serv_option, "-1:-1");
    }
    // open/download files
    if path.starts_with("/open/") {
        let file_path = decode(path).unwrap_or_default().replace("/open/", "/");
        return open_file(&file_path, &serv_option);
    }
    // Other processes
    return process_command(path, &request_header, &serv_option);
}

fn process_command(path: &str, _: &Vec<String>, serv_option: &MservOption) -> (String, Vec<(String, String)>, Option<Box<dyn Iterator<Item = String>>>, Option<Vec<u8>>) {
    if path.starts_with("/scan-dir") {
        scan_movie_dir(serv_option);
        return (String::from("200 OK"), vec![], None, None);
    }
    else if path.starts_with("/transcode-dir") {
        transcode_movie_dir(path, serv_option);
        return (String::from("200 OK"), vec![], None, None);
    }
    else if path.eq("/summary") {
        let summary = serde_json::to_string(&summary::movies_summary(serv_option)).unwrap_or(String::new());
        return (String::from("200 OK"), vec![], None, Some(summary.as_bytes().to_vec()));
    }
    else if path.eq("/all-files-path") {
        let mut files: Vec<String> = vec![];
        let _ = file::scan(&serv_option.base_path, &mut files);
        let files = serde_json::to_string(&files).unwrap_or(String::new());
        return (String::from("200 OK"), vec![], None, Some(files.as_bytes().to_vec()));
    }
    else {
        return (String::from("404 Not Found"), vec![], None, None);
    }
}

fn scan_movie_dir(serv_option: &MservOption) {
    let file_path = serv_option.base_path.to_string();
    if file_path.is_empty() {
        return;
    }
    let mut option = HashMap::new();
    option.insert(String::from("hide-preview"), String::new());
    option.insert(String::from("thread"), String::from("5"));
    option.insert(String::from("provider"), serv_option.provider.clone());

    match serv_option.elastic.as_ref() {
        Some(elastic) => {
            option.insert(String::from("elastic-url"), elastic.url.to_string());
        },
        _ => (),
    }
    let info = Info {
        file_path: file_path.to_string(),
        cmd_options: option,
    };
    thread::spawn(move || info.run());
}

fn transcode_movie_dir(path: &str, serv_option: &MservOption) {
    let extension = ltrim_char(&path.replace("/transcode-dir", ""), '/');

    let file_path = serv_option.base_path.to_string();
    if file_path.is_empty() {
        return;
    }
    let mut option = HashMap::new();
    option.insert(String::from("d"), String::new());
    option.insert(String::from("thread"), String::from("1"));
    if !extension.is_empty() {
        option.insert(String::from("extensions"), extension);
    }

    let transcode = Transcode {
        file_path: file_path.to_string(),
        cmd_options: option,
    };
    thread::spawn(move || transcode.run());    
}

fn process_video(file_path: &String, request_header: &Vec<String>, serv_option: &MservOption) -> (String, Vec<(String, String)>, Option<Box<dyn Iterator<Item = String>>>, Option<Vec<u8>>) {
    let extension = file::get_extension(&file_path);
    if !file::VIDEO_EXTENSIONS.contains(&extension.as_str()) {
        return (String::from("204 No Content"), vec![], None, None);
    }

    let file_path = &get_video_file(&serv_option.base_path, file_path);
    let file_size = file::file_size(&file_path).unwrap_or_default();
    let buffer: u64 = 1_500_000;
    
    let (start_range, _) = get_range_params(&request_header, file_size).unwrap_or((0, buffer));
    let end_range = min(start_range + buffer, file_size) - 1;

    let byte_count = end_range - start_range + 1;
    return (
        String::from("206 Partial Content"), 
        vec![
            (String::from("Content-type"), format!("video/{extension}")),
            (String::from("Accept-Ranges"), String::from("bytes")),
            (String::from("Content-Range"), format!("bytes {start_range}-{end_range}/{file_size}")),
            (String::from("Content-Length"), format!("{}", byte_count)),
        ], 
        None,
        Some(file::read_range(&file_path, start_range, byte_count).unwrap()),
    );
}

fn open_file(file_path: &String, serv_option: &MservOption) -> (String, Vec<(String, String)>, Option<Box<dyn Iterator<Item = String>>>, Option<Vec<u8>>) {
    let file_path = match file::check_file(file_path) {
        Ok(_) => file_path.to_string(),
        _ => {
            let base_path = rtrim_char(&serv_option.base_path, '/');
            format!("{base_path}{file_path}")
        }
    };
    let content = match fs::read(&file_path) {
        Ok(content) => content,
        _ => b"".to_vec()
    };
    if content.is_empty() {
        return (String::from("404 Not Found"), vec![], None, None);
    }
    return (
        String::from("200 OK"), 
        vec![
            (String::from("Content-type"), file::get_mimetype(&file_path)),
        ], 
        None,
        Some(content),
    );
}

/// size: in format width:height, e.g. 600:300, 300:-1 (-1 to keep ratio)
fn process_thumb(file_path: &String, serv_option: &MservOption, size: &str) -> (String, Vec<(String, String)>, Option<Box<dyn Iterator<Item = String>>>, Option<Vec<u8>>) {
    let file_path = match file::check_file(file_path) {
        Ok(_) => file_path.to_string(),
        _ => {
            let base_path = rtrim_char(&serv_option.base_path, '/');
            format!("{base_path}{file_path}")
        }
    };

    let cache_key = digest(&format!("{size}-{file_path}"));
    match cache::get_cache_bytes(&cache_key, ".thumb") {
        Some((_, content)) => return (
            String::from("200 OK"), 
            vec![
                (String::from("Content-type"), String::from("image/jpeg")),
                (String::from("Cache-Control"), String::from("public, max-age=31536000, s-maxage=31536000, immutable")),
            ], 
            None,
            Some(content),
        ),
        None => {
            let extension = file::get_extension(&file_path).to_lowercase();
            let cache_path = cache::get_cache_path(&cache_key, ".thumb");
            let content = if file::VIDEO_EXTENSIONS.contains(&extension.as_str()) {
                // Pick image at random time of video
                let mut rng = rand::thread_rng();
                let at = rng.gen_range(0.05..=0.5);
                movie::generate_thumb(&file_path, &cache_path, size, at)
            } else if file::IMAGE_EXTENSIONS.contains(&extension.as_str()) {
                match fs::read(&file_path) {
                    Ok(content) => content,
                    _ => b"".to_vec()
                }
            } else if file::PDF_EXTENSIONS.contains(&extension.as_str()) {
                pdf::generate_thumb(&file_path, &cache_path, size)
            } else {
                // TODO other format (ms files...)
                // TODO write cache
                b"".to_vec()
            };
            if content.is_empty() {
                return (String::from("404 Not Found"), vec![], None, None);
            }
            return (
                String::from("200 OK"), 
                vec![
                    (String::from("Content-type"), String::from("image/jpeg")),
                    (String::from("Cache-Control"), String::from("public, max-age=31536000, s-maxage=31536000, immutable")),
                ], 
                None,
                Some(content),
            );
        }
    }
}

///
/// TODO: live re-encoding for other format than mp4 or ts
/// https://www.reddit.com/r/rust/comments/iplph5/encoding_decoding_video_streams_in_rust/
fn get_video_file(base_path: &String, file_path: &String) -> String {
    let file_path = rtrim_char(base_path, '/') + file_path;
    if !file_path.ends_with(".mp4") {
        let re = Regex::new(r"(?i)\.[0-9a-z]{2,}$").unwrap();
        let mp4_file_path = re.replace(file_path.as_str(), ".mp4").to_string();
        if let Ok(f) = file::check_file(&mp4_file_path) {
            return f.to_string();
        }
    }
    return file_path.clone();
}
