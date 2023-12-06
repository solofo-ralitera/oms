use std::{path::Path, fs};
use bytes::Bytes;
use sha256::digest;

use super::file::{write_file_content, write_file_bytes};

const CACHE_PATH: &str = "./.cache";

fn base_path() -> &'static Path {
    Path::new(CACHE_PATH)
}

pub fn check_cache(key: &String) -> Option<(String, String)> {
    let hash = digest(key);
    let cache_path = base_path().join(hash);
    if let Ok(contents) = fs::read_to_string(&cache_path) {
        return Some((cache_path.as_path().display().to_string(), contents));
    }
    return None;
}

pub fn write_cache_string(key: &String, content: &String) -> Option<String> {
    let hash = digest(key);
    let cache_path = base_path().join(hash);
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
    let cache_path = base_path().join(hash);

    if let Ok(_) = write_file_bytes(&cache_path, content) {
        return Some(cache_path.as_path().display().to_string());
    }
    return None;
}
