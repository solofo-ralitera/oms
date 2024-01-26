mod metadata;
mod transcode;
mod summary;
mod setting;

use std::thread;
use colored::Colorize;

use crate::helpers::file;
use super::ProcessParam;


pub fn process(path: &str, request_param: &ProcessParam) -> Option<(String, Vec<(String, String)>, Option<Box<dyn Iterator<Item = String>>>, Option<Vec<u8>>)> {
    if path.starts_with("/scan-dir") || path.starts_with("/update-metadata") {
        let update_metadata = if path.contains("/scan-dir") {
            false
        } else if path.contains("/update-metadata") {
            true
        } else {
            false
        };

        let file_path = path
            .replace("/scan-dir", "")
            .replace("/update-metadata", "")
            .trim()
            .to_string();

        return match request_param.verb {
            "GET" => {
                if file_path.is_empty() {
                    // Scan whole dir in a thread (don't wait)
                    let serv_option = request_param.serv_option.clone();
                    thread::spawn(move || if let Err(err) = metadata::scan_media_dir(&file_path, &serv_option, true) {
                        println!("{}", err.to_string().red());
                    });
                    return Some((String::from("200 OK"), vec![], None, None));
                } else {
                    // If scan single file, wait for it
                    return match metadata::scan_media_dir(&file_path, &request_param.serv_option, update_metadata) {
                        Ok(_) => Some((String::from("200 OK"), vec![], None, None)),
                        Err(err) => {
                            println!("{}", err.to_string().red());
                            Some((
                                String::from("500 Internal Server Error"),
                                vec![],
                                None,
                                Some(err.to_string().as_bytes().to_vec())
                            ))
                        },
                    };
                }
            },
            "POST" => match metadata::update_metadata(&file_path, &request_param.serv_option, &request_param.body_content) {
                Ok(_) => Some((String::from("200 OK"), vec![], None, None)),
                Err(err) => {
                    println!("{}", err.to_string().red());
                    Some((
                        String::from("500 Internal Server Error"),
                        vec![],
                        None,
                        Some(err.to_string().as_bytes().to_vec())
                    ))
                }
            },
            _ => Some((String::from("404 Not Found"), vec![], None, None)),
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
