use std::{fs, path::Path};
use bytes::Bytes;
use sha256::digest;

use super::file::{write_file_content, write_file_bytes};

static mut CACHE_PATH: Option<String> = None;

pub fn set_base_path(path: &str) {
    if path.is_empty() {
        // Default path
        unsafe {
            CACHE_PATH = Some(String::from("./.oms/"));
        }
        return;
    }
    unsafe {
        CACHE_PATH = Some(path.to_string());
    }
}

fn base_path() -> String {
    unsafe {
        let path = match CACHE_PATH.clone() {
            Some(p) => p,
            None => String::from("./.oms/"), // Default path
        };
        let _ = fs::create_dir_all(&path);
        return path;
    }
}

pub fn check_cache_path(key: &String) -> Option<String> {
    let hash = digest(key);
    let cache_path = Path::new(&base_path()).join(hash);
    match fs::metadata(&cache_path) {
        Ok(m) => {
            if m.is_file() {
                return Some(cache_path.as_path().display().to_string());
            }
            return None;
        },
        Err(_) => return None,
    }
}

pub fn get_cache(key: &String) -> Option<(String, String)> {
    let hash = digest(key);
    let cache_path = Path::new(&base_path()).join(hash);
    if let Ok(contents) = fs::read_to_string(&cache_path) {
        return Some((cache_path.as_path().display().to_string(), contents));
    }
    return None;
}

pub fn write_cache_string(key: &String, content: &String) -> Option<String> {
    let hash = digest(key);
    let cache_path = Path::new(&base_path()).join(hash);
    if let Ok(_) = write_file_content(
        &cache_path,
        content,
          false
    ) {
        return Some(cache_path.as_path().display().to_string());
    }
    return None;
}

pub fn write_cache_bytes(key: &String, content: &Bytes) -> Option<String> {
    let hash = digest(key);
    let cache_path = Path::new(&base_path()).join(hash);
    if let Ok(_) = write_file_bytes(&cache_path, content) {
        return Some(cache_path.as_path().display().to_string());
    }
    return None;
}
