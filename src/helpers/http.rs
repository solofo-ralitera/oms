use std::io;
use serde::{de::DeserializeOwned, Serialize};
use urlencoding::encode;
use super::cache;

type Result<T> = std::result::Result<T, std::io::Error>;

const CACHE_SUBDIR: &str = ".http";
const CACHE_IMG_SUBDIR: &str = ".img";


pub fn get<T>(url: &String, headers: Vec<(String, String)>, params: Vec<(String, String)>, cache: bool) -> Result<T>
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
        if let Some((_, content)) = cache::get_cache(&cache_key, CACHE_SUBDIR) {
            let result: T = serde_json::from_str(&content).unwrap();
            return Ok(result);
        }
    }
    
    match request.send() {
        Ok(result) => match result.json::<T>() {
            Ok(json) => {
                if let Ok(str_json) = &serde_json::to_string(&json) {
                    cache::write_cache_string(&cache_key, str_json, CACHE_SUBDIR);
                }
                return Ok(json);
            },
            Err(err) => return Err(io::Error::new(
                io::ErrorKind::NotConnected, 
                format!("Request error (json<T>): {err}")
            )),
        },
        Err(err) => return Err(io::Error::new(
            io::ErrorKind::NotConnected, 
            format!("Request send error: {err}")
        ))
    };    
}

pub fn post_body<T>(url: &String, method: &str, headers: &Vec<(String, String)>, post_body: &T) -> Result<String>
where 
    T: Serialize
{
    let mut request = match method {
        "PUT" => {
            reqwest::blocking::Client::new()
                .put(url)
                .header("Content-Type", "application/json")            
        },        
        _ => {
            reqwest::blocking::Client::new()
                .post(url)
                .header("Content-Type", "application/json")            
        },
    };

    for (key, value) in headers {
        request = request.header(key, value);
    }

    let post_body = serde_json::to_string(&post_body).unwrap();

    match request.body(post_body).send() {
        Ok(r) => {
            return Ok(r.text().unwrap_or_default());
        },
        Err(err) => return Err(io::Error::new(
            io::ErrorKind::NotConnected, 
            format!("post_body error: {err}")
        ))
    }
}

pub fn get_image(url: &String) -> Result<String> {
    if let Some(path) = cache::check_cache_path(url, CACHE_IMG_SUBDIR) {
        // TODO: skip here ?
        return Ok(path);
    }

    match reqwest::blocking::get(url) {
        Ok(img_bytes) => {
            match cache::write_cache_bytes(url, &img_bytes.bytes().unwrap(), CACHE_IMG_SUBDIR) {
                Some(path) => Ok(path),
                _ => Ok(String::new()),
            }
        },
        Err(err) => return Err(io::Error::new(
            io::ErrorKind::Unsupported, 
            format!("get_image error: {err}")
        )),
    }
}
