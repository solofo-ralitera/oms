use std::{collections::HashMap, thread};
use regex::Regex;
use crate::{app::commands::{mserv::option::MservOption, transcode::Transcode, info::Info, Runnable}, helpers::{file, ltrim_char, rtrim_char}};
use super::{utils::get_file_path, summary};


pub fn process(path: &str, _: &Vec<String>, serv_option: &MservOption) -> Option<(String, Vec<(String, String)>, Option<Box<dyn Iterator<Item = String>>>, Option<Vec<u8>>)> {
    if path.starts_with("/scan-dir") {
        scan_media_dir(None, serv_option);
        return Some((String::from("200 OK"), vec![], None, None));
    }
    else if path.starts_with("/transcode-dir") {
        transcode_media_dir(path, serv_option);
        return Some((String::from("200 OK"), vec![], None, None));
    }
    else if path.eq("/summary") {
        let summary = serde_json::to_string(&summary::medias_summary(serv_option)).unwrap_or(String::new());
        return Some((String::from("200 OK"), vec![], None, Some(summary.as_bytes().to_vec())));
    }
    else if path.eq("/all-files-path") {
        let mut files: Vec<String> = vec![];
        match file::scan(&serv_option.base_path, &mut files) {
            Ok(_) => {
                let files = serde_json::to_string(&files).unwrap_or(String::new());
                return Some((String::from("200 OK"), vec![], None, Some(files.as_bytes().to_vec())));
            },
            Err(_) => {
                return Some((String::from("404 Not Found"), vec![], None, None));
            }
        }
    }
    None
}

fn scan_media_dir(file_path: Option<String>, serv_option: &MservOption) {
    let file_path = match file_path {
        None => serv_option.base_path.to_string(),
        Some(p) => p,
    };
    if file_path.is_empty() {
        return;
    }
    let mut option = HashMap::new();
    option.insert(String::from("hide-preview"), String::new());
    option.insert(String::from("thread"), String::from("5"));
    option.insert(String::from("provider"), serv_option.provider.clone());
    option.insert(String::from("base-path"), serv_option.base_path.clone());

    if let Some(elastic) = serv_option.elastic.as_ref() {
        option.insert(String::from("elastic-url"), elastic.url.to_string());
    }

    let file_path_thread = file_path.clone();
    match thread::spawn(move || Info {
        file_path: file_path_thread.to_string(),
        cmd_options: option,
    }.run()).join() {
        Ok(_) => {
            println!("Scan finished on {file_path}");
        },
        _ => (),
    }
}

fn transcode_media_dir(path: &str, serv_option: &MservOption) {
    let path = ltrim_char(&path.replace("/transcode-dir", ""), '/');
    
    let mut file_path = serv_option.base_path.to_string();
    if file_path.is_empty() {
        return;
    }

    let re_extension = Regex::new("^(?im)[0-9a-z]{2,5}$").unwrap();

    // Transcode option
    let mut option = HashMap::new();
    option.insert(String::from("d"), String::new());
    option.insert(String::from("output"), serv_option.transcode_output.clone());
    option.insert(String::from("thread"), serv_option.transcode_thread.to_string());
    
    if path.is_empty() {
        // transcode all file in base_path
        // do not transcode known streaming formats
        let mut extension = file::VIDEO_EXTENSIONS.join(",");
        extension = extension.replace("mp4", "");
        extension = extension.replace("ts", "");
        extension = extension.replace("webm", "");        
        extension = extension.replace(",,", ",");
        extension = rtrim_char(&extension, ',');
        extension = ltrim_char(&extension, ',');
        option.insert(String::from("extensions"), extension);
    }
    else if re_extension.is_match(&path) {
        // if path is an extension => transcode all file with this extension in base_path
        if !file::VIDEO_EXTENSIONS.contains(&path.to_lowercase().as_str()) {
            return;
        }
        option.insert(String::from("extensions"), path);
    }
    else {
        // if path is a file => transcode this file to transcode_output
        file_path = get_file_path(&serv_option.base_path, &path).unwrap_or_default();
        if file_path.is_empty() {
            return;
        }
        if !file::is_video_file(&file_path) {
            return;
        }
    };
    
    let file_path_thread = file_path.clone();
    match thread::spawn(move || Transcode {
        file_path: file_path_thread.to_string(),
        cmd_options: option,
    }.run()).join() {
        Ok(_) => {
            println!("Transcode finished on {file_path}");
            // update info after transcode
            scan_media_dir(file::get_file_dir(&file_path), serv_option);
        },
        _ => (),
    }
}
