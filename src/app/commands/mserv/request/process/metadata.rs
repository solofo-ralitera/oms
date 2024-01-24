use std::{collections::HashMap, io, thread};
use colored::Colorize;
use crate::{app::commands::{mserv::{option::MservOption, request::utils}, info::Info, Runnable}, helpers::{file, media::{video, self}}};

type Result<T> = std::result::Result<T, std::io::Error>;

pub fn scan_media_dir(file_path: &String, serv_option: &MservOption, update_metadata: bool) {
    let file_path = if file_path.is_empty() {
        serv_option.base_path.to_string()
    } else {
        match utils::get_file_path(&serv_option.base_path, &file_path.replace(&serv_option.base_path, "")) {
            Some(path) => {
                if file::is_video_file(&path) {
                    video::result::clear_cache(&path);
                }
                path
            },
            None => String::new(),
        }

    };
    if file_path.is_empty() {
        println!("Scan media: file_path does not exist");
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

pub fn update_metadata(file_path: &String, serv_option: &MservOption, body_content: &String) -> Result<bool> {
    let file_path = if file_path.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput, 
            format!("Update metadata: error file_path is empty")
        ))
    } else {
        utils::get_file_path(&serv_option.base_path, &file_path.replace(&serv_option.base_path, "")).unwrap_or_default()
    };

    if file_path.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput, 
            format!("Update metadata: error file_path is empty")
        ))
    }
    if file::is_video_file(&file_path) {
        return video::metadata::VideoMetadata::write_from_body_content(&file_path, body_content);
    } else if file::is_pdf_file(&file_path) {
        return media::pdf::metadata::PdfMetadata::write_from_body_content(&file_path, body_content);
    }
    return Err(io::Error::new(
        io::ErrorKind::Unsupported, 
        format!("Update metadata: error file not supported")
    ))
}
