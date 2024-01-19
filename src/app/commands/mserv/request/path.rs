use std::{cmp::min, fs};
use crate::helpers::{file, input::get_range_params, media::video};
use super::{utils, ProcessParam};


pub fn process(path: &str, request_param: &ProcessParam) -> Option<(String, Vec<(String, String)>, Option<Box<dyn Iterator<Item = String>>>, Option<Vec<u8>>)> {
    // Video files
    if path.starts_with("/stream/") {
        let file_path = utils::get_file_path(&request_param.serv_option.base_path, &path.replace("/stream/", "/"));
        if file_path.is_none() {
            return Some((String::from("404 Not Found"), vec![], None, None));
        }
        return Some(process_stream(&file_path.unwrap(), &request_param.request_header));
    }
    // Thumb files (width=300)
    if path.starts_with("/thumb/") {
        let file_path = utils::get_file_path(&request_param.serv_option.base_path, &path.replace("/thumb/", "/"));
        if file_path.is_none() {
            return Some((String::from("404 Not Found"), vec![], None, None));
        }
        return Some(utils::process_thumb(&file_path.unwrap(), "300:-1"));
    }
    // Poster files (no resize)
    if path.starts_with("/poster/") {
        let file_path = utils::get_file_path(&request_param.serv_option.base_path, &path.replace("/poster/", "/"));
        if file_path.is_none() {
            return Some((String::from("404 Not Found"), vec![], None, None));
        }
        return Some(utils::process_thumb(&file_path.unwrap(), "-1:-1"));
    }
    // open/download files
    if path.starts_with("/open/") {
        let file_path = utils::get_file_path(&request_param.serv_option.base_path, &path.replace("/open/", "/"));
        if file_path.is_none() {
            return Some((String::from("404 Not Found"), vec![], None, None));
        }
        return Some(open_file(&file_path.unwrap()));
    }
    None
}

fn open_file(file_path: &String) -> (String, Vec<(String, String)>, Option<Box<dyn Iterator<Item = String>>>, Option<Vec<u8>>) {
    let content = match fs::read(&file_path) {
        Ok(content) => content,
        _ => b"".to_vec()
    };

    let file_size: u64 = file::file_size(&file_path).unwrap_or_default();
    return (
        String::from("200 OK"), 
        vec![
            (String::from("Content-type"), file::get_mimetype(&file_path)),
            (String::from("Content-Length"), format!("{file_size}")),
        ], 
        None,
        Some(content),
    );
}

fn process_stream(file_path: &String, request_header: &Vec<String>) -> (String, Vec<(String, String)>, Option<Box<dyn Iterator<Item = String>>>, Option<Vec<u8>>) {
    let extension = file::get_extension(&file_path);
    
    // TODO test file > 0

    if !file::VIDEO_EXTENSIONS.contains(&extension.as_str()) && !file::AUDIO_EXTENSIONS.contains(&extension.as_str()) {
        return (String::from("204 No Content"), vec![], None, None);
    }

    let file_path = &video::get_video_file(file_path);
    let file_size = file::file_size(&file_path).unwrap_or_default();
    let buffer: u64 = 1_500_000;
    
    let (start_range, _) = get_range_params(&request_header, file_size).unwrap_or((0, buffer));
    let end_range = min(start_range + buffer, file_size);
    let end_range = if end_range > 0 {
        end_range - 1
    } else {
        0
    };
    
    let byte_count = if end_range >= start_range {
        end_range - start_range + 1
    } else {
        0
    };

    return (
        String::from("206 Partial Content"), 
        vec![
            (String::from("Content-type"), file::get_mimetype(&file_path)),
            (String::from("Accept-Ranges"), String::from("bytes")),
            (String::from("Content-Range"), format!("bytes {start_range}-{end_range}/{file_size}")),
            (String::from("Content-Length"), format!("{}", byte_count)),
        ], 
        None,
        Some(file::read_range(&file_path, start_range, byte_count).unwrap_or(b"".to_vec())),
    );
}