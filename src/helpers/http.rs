use std::io;

use serde::{de::DeserializeOwned, Serialize};
use urlencoding::encode;

use super::cache::{self, write_cache_string, write_cache_bytes};


pub fn get<T>(url: &String, headers: Vec<(String, String)>, params: Vec<(String, String)>, cache: bool) -> Result<T, io::Error>
where
    T: DeserializeOwned + Serialize,
{
    let mut encoded_params = vec![];
    for (key, value) in &params {
        encoded_params.push(format!("{key}={}", encode(value).into_owned()));
    }
    let url = format!("{url}?{}", encoded_params.join("&"));
    let mut request = reqwest::blocking::Client::new()
        .get(&url);

    let mut cache_key = format!("{url}");

    for (key, value) in &headers {
        request = request.header(key, value);
        cache_key.push_str(&format!("{key}:{value},"));
    }

    if cache == true {
        if let Some((_, content)) = cache::check_cache(&cache_key) {
            let result: T = serde_json::from_str(&content).unwrap();
            return Ok(result);
        }
    }
    
    let result = request.send()
        .unwrap()
        .json::<T>()
        .unwrap();
        
    write_cache_string(&cache_key, &serde_json::to_string(&result).unwrap().to_string());

    return Ok(result);
}

pub fn get_image(url: &String) -> Result<String, io::Error> {
    if let Some((path, _)) = cache::check_cache(url) {
        return Ok(path);
    }
    let img_bytes = reqwest::blocking::get(url).unwrap().bytes().unwrap();
    match write_cache_bytes(url, &img_bytes) {
        Some(path) => Ok(path),
        _ => Ok(String::new()),
    }
}
