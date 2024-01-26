use std::{collections::HashMap, thread};
use regex::Regex;
use crate::{app::commands::{mserv::{option::MservOption, request::utils}, transcode::Transcode, Runnable}, helpers::{self, file}};

pub fn transcode_media_dir(path: &str, serv_option: &MservOption) {
    let path = helpers::ltrim_char(&path.replace("/transcode-dir", ""), '/');
    
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

    if re_extension.is_match(&path) {
        // if path is an extension => transcode all file with this extension in base_path
        if !file::VIDEO_EXTENSIONS.contains(&path.to_lowercase().as_str()) {
            return;
        }
        option.insert(String::from("extensions"), path);
    }
    else if !path.is_empty(){
        // if path is a file => transcode this file to transcode_output
        file_path = utils::get_file_path(&serv_option.base_path, &path).unwrap_or_default();
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
