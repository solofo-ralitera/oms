use std::{fs, path::Path};
use bytes::Bytes;
use regex::Regex;
use serde::Serialize;
use sha256::digest;
use super::{file::{write_file_content, write_file_bytes}, db::kvstore::KVStore};
use once_cell::sync::Lazy;


struct Cache {
    base_path: String,
    kv_store: Option<KVStore>,
}

impl Cache {
    fn base_file_path(&self, file_name: &String) -> String {
        let cache_path = Path::new(&self.base_path);
        let _ = fs::create_dir_all(&cache_path);
        let cache_path = cache_path.join(file_name);
        return cache_path.as_path().display().to_string();
    }    

    fn set_kv(&mut self) {
        self.kv_store = Some(KVStore::new(self.base_file_path(&"oms.cab".to_string())));
    }

    pub fn set_base_path(&mut self, path: &str) {
        self.base_path = path.to_string();
        self.set_kv();
    }
}

impl Cache {
    fn get_cache_path(&self, key: &String, subdir: &str) -> String {
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

        let cache_path = Path::new(&self.base_path).join(subdir);
        let _ = fs::create_dir_all(&cache_path);
        let cache_path = cache_path.join(hash);
        return cache_path.as_path().display().to_string();
    }

    pub fn check_cache_path(&self, key: &String, subdir: &str) -> Option<String> {
        let cache_path = self.get_cache_path(key, subdir);
        match fs::metadata(&cache_path) {
            Ok(m) => match m.is_file() {
                true => return Some(cache_path),
                false => return None,
            },
            Err(_) => return None,
        }
    }    
}

impl Cache {
    pub fn kv_get(&mut self, key: &String) -> Option<String> {
        return self.kv_store.as_mut()?.get(key);
    }

    pub fn kv_add(&mut self, key: &String, value: &String) {
        match self.kv_store.as_mut() {
            None => (),
            Some(kv) => {
                kv.add(key, value);
            }
        }
    }    
}

impl Cache {
    pub fn get_cache(&self, key: &String, subdir: &str) -> Option<(String, String)> {
        let cache_path = self.get_cache_path(key, subdir);
        if let Ok(contents) = fs::read_to_string(&cache_path) {
            return Some((cache_path, contents));
        }
        return None;
    }
    
    pub fn clear_cache(&self, key: &String, subdir: &str) {
        let cache_path = self.get_cache_path(key, subdir);
        let _ = fs::remove_file(cache_path);
    }

    pub fn get_cache_bytes(&self, key: &String, subdir: &str) -> Option<(String, Vec<u8>)> {
        let cache_path = self.get_cache_path(key, subdir);
        if let Ok(contents) = fs::read(&cache_path) {
            return Some((cache_path, contents));
        }
        return None;
    }

    pub fn write_cache_string(&self, key: &String, content: &String, subdir: &str) -> Option<String> {
        let cache_path = self.get_cache_path(key, subdir);
        if let Ok(_) = write_file_content(&Path::new(&cache_path), content,false) {
            return Some(cache_path);
        }
        return None;
    }
    
    pub fn write_cache_json<T>(&self, key: &String, json: T, subdir: &str) -> Option<String>
    where
        T: Serialize
    {
        let str_json = serde_json::to_string(&json).unwrap();
        return self.write_cache_string(key, &str_json, subdir);
    }
    
    pub fn write_cache_bytes(&self, key: &String, content: &Bytes, subdir: &str) -> Option<String> {
        let cache_path = self.get_cache_path(key, subdir);
        if let Ok(_) = write_file_bytes(Path::new(&cache_path), content) {
            return Some(cache_path);
        }
        return None;
    }
    
    pub fn append_cache_content(&self, key: &String, content: &String, subdir: &str) -> Option<String> {
        let cache_path = self.get_cache_path(key, subdir);
        if let Ok(_) = write_file_content(Path::new(&cache_path), content, true) {
            return Some(cache_path);
        }
        return None;
    }    
}


static mut CACHE: Lazy<Cache> = Lazy::new(|| {
    Cache {
        base_path: String::from("./.oms/"),
        kv_store: None,
    }
});

pub fn set_base_path(path: &str) {
    unsafe {
        CACHE.set_base_path(path);
    }
}

pub fn get_cache(key: &String, subdir: &str) -> Option<(String, String)> {
    unsafe {
        return CACHE.get_cache(key, subdir);
    }
}

pub fn clear_cache(key: &String, subdir: &str) {
    unsafe {
        CACHE.clear_cache(key, subdir);
    }
}

pub fn get_cache_bytes(key: &String, subdir: &str) -> Option<(String, Vec<u8>)> {
    unsafe {
        return CACHE.get_cache_bytes(key, subdir);
    }
}

pub fn get(key: &String) -> Option<String> {
    unsafe {
        return CACHE.kv_get(key);
    }
}

pub fn add(key: &String, value: &String) {
    unsafe {
        CACHE.kv_add(key, value);
    }
}

pub fn get_cache_path(key: &String, subdir: &str) -> String {
    unsafe {
        return CACHE.get_cache_path(key, subdir);
    }
}

pub fn write_cache_json<T: Serialize>(key: &String, json: T, subdir: &str) -> Option<String> {
    unsafe {
        return CACHE.write_cache_json(key, json, subdir);
    }
}

pub fn append_cache_content(key: &String, content: &String, subdir: &str) -> Option<String> {
    unsafe {
        return CACHE.append_cache_content(key, content, subdir);
    }
}

pub fn check_cache_path(key: &String, subdir: &str) -> Option<String> {
    unsafe {
        return CACHE.check_cache_path(key, subdir);
    }
}    

pub fn write_cache_string(key: &String, content: &String, subdir: &str) -> Option<String> {
    unsafe {
        return CACHE.write_cache_string(key, content, subdir);
    }
}

pub fn write_cache_bytes(key: &String, content: &Bytes, subdir: &str) -> Option<String> {
    unsafe {
        return CACHE.write_cache_bytes(key, content, subdir);
    }
}
