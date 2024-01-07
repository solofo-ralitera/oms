mod summary;
mod path;
mod process;
mod utils;
mod staticfile;

use urlencoding::decode;
use super::option::MservOption;

pub struct ProcessParam<'a> {
    pub path: &'a str,
    pub verb: &'a str,
    pub request_header: &'a Vec<String>,
    pub serv_option: &'a MservOption,
}

///
/// Return: status: 200 OK, headers, content
pub fn process(ProcessParam {path, verb, request_header, serv_option}: ProcessParam) -> (String, Vec<(String, String)>, Option<Box<dyn Iterator<Item = String>>>, Option<Vec<u8>>) {
    if verb == "OPTIONS" {
        return (String::new(), vec![], None, None);
    }
    let path = decode(path).unwrap_or_default();
    let path: &str = path.as_ref();
    let path = path.replace("//", "/");
    let path = path.as_str();

    return if let Some(r) = staticfile::process(path, request_header, &serv_option) {
        r 
    } else if let Some(r) = path::process(path, request_header, &serv_option) {
        r 
    } else if let Some(r) = process::process(path, &request_header, &serv_option) {
        r
    } else {
        (String::from("404 Not Found"), vec![], None, None)
    };
}
