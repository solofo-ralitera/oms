pub mod string;
pub mod file;
pub mod pdf;
pub mod http;
pub mod output;
pub mod input;
pub mod movie;
pub mod cache;
pub mod db;

use std::{thread, time::Duration};

use regex::Regex;

/// Pause the thread for x millis
/// 
/// # Arguments
///
/// * `millis` - pause duration in millis
/// 
/// # Examples
/// 
/// ```
/// use oms::helpers;
/// helpers::sleep(0);
/// ```
pub fn sleep(millis: u64) {
    thread::sleep(Duration::from_millis(millis));
}

pub fn rtrim_char(str: &String, char: char) -> String {
    let mut re_string = String::new();
    re_string.push(char);
    re_string.push('$');
    let re = Regex::new(re_string.as_str()).unwrap();
    return re.replace(str, "").to_string();
}
