pub mod string;
pub mod file;
pub mod pdf;
pub mod http;
pub mod output;
pub mod input;
pub mod movie;
pub mod cache;
pub mod db;
pub mod threadpool;
pub mod command;

use std::{thread, time::Duration};

use regex::Regex;

/// Pause the thread for x millis
/// 
/// # Arguments
///
/// * `millis` - pause duration in millis
pub fn sleep(millis: u64) {
    thread::sleep(Duration::from_millis(millis));
}

/// Strig char from the end of a string
/// 
/// # Arguments
///
/// * `str` - the input string
/// * `char` - the characters to be stripped
/// 
/// # Examples
/// 
/// ```
/// use oms::helpers;
/// 
/// let my_string = String::from("http://localhost/osm/");
/// let stripperd_string = helpers::rtrim_char(&my_string, '/');
/// assert_eq!("http://localhost/osm", stripperd_string);
/// ```
pub fn rtrim_char(str: &String, char: char) -> String {
    let mut re_string = String::new();
    re_string.push(char);
    re_string.push('$');
    let re = Regex::new(re_string.as_str()).unwrap();
    return re.replace(str, "").to_string();
}

pub fn ltrim_char(str: &String, char: char) -> String {
    let mut re_string = String::new();
    re_string.push('^');
    re_string.push(char);
    let re = Regex::new(re_string.as_str()).unwrap();
    return re.replace(str, "").to_string();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rtrim_char_slash() {
        let my_string = String::from("http://localhost/osm/");
        assert_eq!("http://localhost/osm", rtrim_char(&my_string, '/'));
    }

    #[test]
    fn rtrim_char_notfound() {
        let my_string = String::from("http://localhost/osm");
        assert_eq!("http://localhost/osm", rtrim_char(&my_string, '/'));
    }

}