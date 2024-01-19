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
    pub body_content: &'a String,
}

///
/// Return: status: 200 OK, headers, content
pub fn process(request_param: &ProcessParam) -> (String, Vec<(String, String)>, Option<Box<dyn Iterator<Item = String>>>, Option<Vec<u8>>) {
    if request_param.verb.eq("OPTIONS") {
        return (String::new(), vec![], None, None);
    }
    let path = decode(request_param.path).unwrap_or_default();
    let path: &str = path.as_ref();
    let path = path.replace("//", "/");
    let path: &str = path.as_str();

    return if let Some(r) = staticfile::process(path, request_param) {
        r 
    } else if let Some(r) = path::process(path, request_param) {
        r 
    } else if let Some(r) = process::process(path, request_param) {
        r
    } else {
        (String::from("404 Not Found"), vec![], None, None)
    };
}
