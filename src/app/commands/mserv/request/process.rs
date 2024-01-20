mod metadata;
mod transcode;
mod summary;
mod setting;

use std::thread;
use crate::helpers::file;
use super::ProcessParam;


pub fn process(path: &str, request_param: &ProcessParam) -> Option<(String, Vec<(String, String)>, Option<Box<dyn Iterator<Item = String>>>, Option<Vec<u8>>)> {
    if path.starts_with("/scan-dir") {
        let file_path = path
            .replace("/scan-dir", "")
            .trim()
            .to_string();
        let serv_option = request_param.serv_option.clone();
        if file_path.is_empty() {
            thread::spawn(move || metadata::scan_media_dir(&file_path, &serv_option, false));
        } else {
            metadata::scan_media_dir(&file_path, &serv_option, false);
        };
        return Some((String::from("200 OK"), vec![], None, None));
    }
    else if path.starts_with("/update-metadata") {
        let file_path = path
            .replace("/update-metadata", "")
            .trim()
            .to_string();
        match request_param.verb {
            "GET" => {
                let serv_option = request_param.serv_option.clone();
                thread::spawn(move || metadata::scan_media_dir(&file_path, &serv_option, true));
                return Some((String::from("200 OK"), vec![], None, None));
            },
            "POST" => {
                if metadata::update_metadata(&file_path, &request_param.serv_option, &request_param.body_content) == true {
                    return Some((String::from("200 OK"), vec![], None, None));
                }
                return Some((String::from("500 Internal Server Error"), vec![], None, None));
            },
            _ => {
                return Some((String::from("404 Not Found"), vec![], None, None));
            },
        };
    }
    else if path.starts_with("/transcode-dir") {
        transcode::transcode_media_dir(path, request_param.serv_option);
        return Some((String::from("200 OK"), vec![], None, None));
    }
    else if path.eq("/summary") {
        let summary = serde_json::to_string(&summary::medias_summary(request_param.serv_option)).unwrap_or(String::new());
        return Some((String::from("200 OK"), vec![], None, Some(summary.as_bytes().to_vec())));
    }
    else if path.eq("/all-files-path") {
        let mut files: Vec<String> = vec![];
        match file::scan(&request_param.serv_option.base_path, &mut files) {
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
        return Some((String::from("200 OK"), vec![], None, Some(setting::get_service_log(request_param.serv_option).as_bytes().to_vec())));
    }
    else if path.eq("/prerequistes") {
        let prerequistes = serde_json::to_string(&setting::get_prerequistes_version(request_param.serv_option));
        return Some((String::from("200 OK"), vec![], None, Some(prerequistes.unwrap_or(String::new()).as_bytes().to_vec())));
    }
    
    None
}
