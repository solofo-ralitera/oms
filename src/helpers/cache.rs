use std::{fs, path::Path};
use bytes::Bytes;
use regex::Regex;
use serde::Serialize;
use sha256::digest;

use super::file::{write_file_content, write_file_bytes};

static mut CACHE_PATH: Option<String> = None;

pub fn set_base_path(path: &str) {
    unsafe {
        if !path.is_empty() {
            CACHE_PATH = Some(path.to_string());
            return;
        }
        CACHE_PATH = None;
    }
}

pub fn base_path() -> String {
    unsafe {
        let path = match CACHE_PATH.clone() {
            Some(p) => p,
            None => String::from("./.oms/"), // Default path
        };
        return path;
    }
}

pub fn base_file_path(file_name: &String) -> String {
    let path = base_path().to_string();
    let cache_path = Path::new(&path);
    let _ = fs::create_dir_all(&cache_path);
    let cache_path = cache_path.join(file_name);
    return cache_path.as_path().display().to_string();
}

fn get_cache_path(key: &String, subdir: &str) -> String {
    let mut hash = digest(key);

    // If key is a date (YYYY-MM-DD): keep original key as hash
    let re_date = Regex::new("^[0-9]{4}-[0-9]{2}-[0-9]{2}$").unwrap();
    if re_date.is_match(&key) {
        hash = key.clone();
    }
    // If key is hash like, keep also
    let re_hash = Regex::new("^[a-z0-9]{60,70}$").unwrap();
    if re_hash.is_match(key) {
        hash = key.clone();
    }

    let cache_path = Path::new(&base_path()).join(subdir);
    let _ = fs::create_dir_all(&cache_path);
    let cache_path = cache_path.join(hash);
    return cache_path.as_path().display().to_string();
}

pub fn check_cache_path(key: &String, subdir: &str) -> Option<String> {
    let cache_path = get_cache_path(key, subdir);
    match fs::metadata(&cache_path) {
        Ok(m) => match m.is_file() {
            true => return Some(cache_path),
            false => return None,
        },
        Err(_) => return None,
    }
}

pub fn get_cache(key: &String, subdir: &str) -> Option<(String, String)> {
    let cache_path = get_cache_path(key, subdir);
    if let Ok(contents) = fs::read_to_string(&cache_path) {
        return Some((cache_path, contents));
    }
    return None;
}

pub fn write_cache_string(key: &String, content: &String, subdir: &str) -> Option<String> {
    let cache_path = get_cache_path(key, subdir);
    if let Ok(_) = write_file_content(&Path::new(&cache_path), content,false) {
        return Some(cache_path);
    }
    return None;
}

pub fn write_cache_json<T>(key: &String, json: T, subdir: &str) -> Option<String>
where
    T: Serialize
{
    let str_json = serde_json::to_string(&json).unwrap();
    return write_cache_string(key, &str_json, subdir);
}

pub fn write_cache_bytes(key: &String, content: &Bytes, subdir: &str) -> Option<String> {
    let cache_path = get_cache_path(key, subdir);
    if let Ok(_) = write_file_bytes(Path::new(&cache_path), content) {
        return Some(cache_path);
    }
    return None;
}

pub fn append_cache_content(key: &String, content: &String, subdir: &str) -> Option<String> {
    let cache_path = get_cache_path(key, subdir);
    if let Ok(_) = write_file_content(Path::new(&cache_path), content, true) {
        return Some(cache_path);
    }
    return None;
}
