use std::{collections::HashMap, thread};
use colored::Colorize;
use regex::Regex;
use crate::{app::commands::{mserv::option::MservOption, transcode::Transcode, info::Info, Runnable}, helpers::{file::{self, get_file_name}, ltrim_char, rtrim_char, command}};
use super::{utils::get_file_path, summary};


pub fn process(path: &str, _: &Vec<String>, serv_option: &MservOption) -> Option<(String, Vec<(String, String)>, Option<Box<dyn Iterator<Item = String>>>, Option<Vec<u8>>)> {
    if path.starts_with("/scan-dir") {
        let path = path.to_string().clone();
        let serv_option = serv_option.clone();
        thread::spawn(move || scan_media_dir(&path, &serv_option, false));
        return Some((String::from("200 OK"), vec![], None, None));
    }
    else if path.starts_with("/update-metadata") {
        let path = path.to_string().clone();
        let serv_option = serv_option.clone();
        thread::spawn(move || scan_media_dir(&path, &serv_option, true));
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
    else if path.eq("/service-log") {
        return Some((String::from("200 OK"), vec![], None, Some(get_service_log(serv_option).as_bytes().to_vec())));
    }
    else if path.eq("/prerequistes") {
        let prerequistes = serde_json::to_string(&get_prerequistes_version(serv_option));
        return Some((String::from("200 OK"), vec![], None, Some(prerequistes.unwrap_or(String::new()).as_bytes().to_vec())));
    }
    
    None
}

fn scan_media_dir(path: &str, serv_option: &MservOption, update_metadata: bool) {
    let file_path = path
        .replace("/scan-dir", "")
        .replace("/update-metadata", "")
        .trim()
        .to_string();
    let file_path = if file_path.is_empty() {
        serv_option.base_path.to_string()
    } else {
        get_file_path(&serv_option.base_path, &file_path.replace(&serv_option.base_path, "")).unwrap_or_default()
    };

    if file_path.is_empty() {
        return;
    }
    let mut option = HashMap::new();
    option.insert(String::from("hide-preview"), String::new());
    option.insert(String::from("thread"), serv_option.transcode_thread.to_string().clone());
    option.insert(String::from("provider"), serv_option.provider.clone());
    option.insert(String::from("base-path"), serv_option.base_path.clone());
    if update_metadata == true {
        option.insert(String::from("update-metadata"), String::new());
        option.insert(String::from("thread"), "1".to_string());
    }

    if let Some(elastic) = serv_option.elastic.as_ref() {
        option.insert(String::from("elastic-url"), elastic.url.to_string());
        // Drop whole index
        if file_path.eq(&serv_option.base_path) && update_metadata == false {
            elastic.drop_index();
        }
    }
    let file_path_thread = file_path.clone();
    match thread::spawn(move || Info {
        file_path: file_path_thread.to_string(),
        cmd_options: option,
    }.run()).join() {
        Ok(_) => println!("Scan finished on {file_path}"),
        _ => println!("{} {}", "Scan finished with error on".red(), file_path.red()),
    }
}

fn get_service_log(serv_option: &MservOption) -> String {
    // journalctl -u movies.service
    let servicename = get_file_name(&rtrim_char(&rtrim_char(&serv_option.base_path, '/'), '\\'));
    return command::exec(
        "journalctl",
        ["-u", &format!("{servicename}.service")]
    );
}

fn get_prerequistes_version(serv_option: &MservOption) -> HashMap<&str, String> {
    let mut result = HashMap::new();
    result.insert("ffmpeg", command::exec("ffmpeg",["-version"]).split('\n').next().unwrap_or("").to_string());
    result.insert("ffprobe", command::exec("ffprobe",["-version"]).split('\n').next().unwrap_or("").to_string());
    result.insert("convert", command::exec("convert",["-version"]).split('\n').next().unwrap_or("").to_string());
    result.insert("exiftool", command::exec("exiftool",["-ver"]).split('\n').next().unwrap_or("").to_string());
    result.insert("elastic", match serv_option.elastic.as_ref() {
        Some(elastic) => elastic.url.to_string(),
        None => String::new(),
    });
    return result;
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
        },
        _ => (),
    }
}
