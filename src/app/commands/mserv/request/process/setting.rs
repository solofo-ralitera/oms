use std::collections::HashMap;

use crate::{app::commands::mserv::option::MservOption, helpers::{file, self}};


pub fn get_service_log(serv_option: &MservOption) -> String {
    // journalctl -u movies.service
    let servicename = file::get_file_name(&helpers::rtrim_char(&helpers::rtrim_char(&serv_option.base_path, '/'), '\\'));
    return helpers::command::exec(
        "journalctl",
        ["-u", &format!("{servicename}.service")]
    );
}

pub fn get_prerequistes_version(serv_option: &MservOption) -> HashMap<&str, String> {
    let mut result = HashMap::new();
    result.insert("ffmpeg", helpers::command::exec("ffmpeg",["-version"]).split('\n').next().unwrap_or("").to_string());
    result.insert("ffprobe", helpers::command::exec("ffprobe",["-version"]).split('\n').next().unwrap_or("").to_string());
    result.insert("convert", helpers::command::exec("convert",["-version"]).split('\n').next().unwrap_or("").to_string());
    result.insert("exiftool", helpers::command::exec("exiftool",["-ver"]).split('\n').next().unwrap_or("").to_string());
    result.insert("elastic", match serv_option.elastic.as_ref() {
        Some(elastic) => elastic.url.to_string(),
        None => String::new(),
    });
    return result;
}
