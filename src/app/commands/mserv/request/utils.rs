use std::{fs, path::Path};
use rand::Rng;
use sha256::digest;
use crate::helpers::{file, ltrim_char, rtrim_char, cache, media::{video, pdf}};


/// Return the required file_path, with checking traversal
pub fn get_file_path(base_path: &String, file_path: &String) -> Option<String> {
    let file_path = Path::new(&rtrim_char(base_path, '/')).join(ltrim_char(file_path, '/')).as_path().display().to_string();
    // Test is file existe and return canonical path
    let file_path = file::check_file(&file_path);
    if file_path.is_err() {
        return None;
    }

    let file_path = file_path.unwrap();

    // test base path for traversal
    if file_path.starts_with(base_path) {
        return Some(file_path);
    }
    None
}

/// size: in format width:height, e.g. 600:300, 300:-1 (-1 to keep ratio)
pub fn process_thumb(file_path: &String, size: &str) -> (String, Vec<(String, String)>, Option<Box<dyn Iterator<Item = String>>>, Option<Vec<u8>>) {
    let cache_key = digest(&format!("{size}-{file_path}"));
    match cache::get_cache_bytes(&cache_key, ".thumb") {
        None => {
            let cache_path = cache::get_cache_path(&cache_key, ".thumb");
            let content = if file::is_video_file(file_path) {
                // Pick image at random time of video
                let mut rng = rand::thread_rng();
                let at = rng.gen_range(0.05..=0.5);
                video::generate_thumb(&file_path, &cache_path, size, at)
            } else if file::is_image_file(file_path) {
                match fs::read(&file_path) {
                    Ok(content) => content,
                    _ => b"".to_vec()
                }
            } else if file::is_pdf_file(file_path) {
                pdf::generate_thumb(&file_path, &cache_path, size)
            } else {
                // TODO other format (ms files...)
                // TODO write cache
                b"".to_vec()
            };
            if content.is_empty() {
                return (String::from("404 Not Found"), vec![], None, None);
            }
            return (
                String::from("200 OK"), 
                vec![
                    (String::from("Content-type"), String::from("image/jpeg")),
                    (String::from("Cache-Control"), String::from("public, max-age=31536000, s-maxage=31536000, immutable")),
                ], 
                None,
                Some(content),
            );
        },
        Some((_, content)) => return (
            String::from("200 OK"), 
            vec![
                (String::from("Content-type"), String::from("image/jpeg")),
                (String::from("Cache-Control"), String::from("public, max-age=31536000, s-maxage=31536000, immutable")),
            ], 
            None,
            Some(content),
        ),
    }
}
