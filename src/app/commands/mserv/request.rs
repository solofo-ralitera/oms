use crate::helpers::file;
use urlencoding::decode;
use std::{io, cmp::min};


///
/// Return: status: 200 OK, headers, content
/// 
pub fn process(path: &str, verb: &str, request_header: &Vec<String>) -> (String, Vec<(String, String)>, Option<Box<dyn Iterator<Item = String>>>, Option<Vec<u8>>) {
    if verb == "OPTION" {
        return (String::new(), vec![], None, None);
    }
    let path = if path == "/" {
        "/assets/index.html"
    } else if path == "/favicon.ico" {
        "/assets/favicon.ico"
    } else {
        path
    };
    let mut file_path = String::new();
    if path.starts_with("/assets/") {
        file_path = format!("./resources/mserv{}", decode(path).unwrap_or_default());
    }
    if path.starts_with("/movie/") {
        file_path = decode(path).unwrap_or_default().replace("/movie/", "/");
    }
    if file_path.is_empty() {
        return (String::from("404 Not Found"), vec![], None, None);
    }
    let mime = file::get_mimetype(path).to_string();
    // Binary content
    if mime.starts_with("image") {
        return (
            String::from("200 OK"), 
            vec![
                (String::from("Content-type"), file::get_mimetype(path).to_string()),
            ], 
            None,
            Some(file::read_buf(&file_path)),
        );
    } else if mime.starts_with("video") {
        let file_size = file::file_size(&file_path).unwrap_or_default();
        let buffer: u64 = 1_500_000;
        
        let mut start_range: u64 = 0;
        let mut end_range: u64 = 0;

        let range = request_header.iter().filter(|line| line.starts_with("Range:")).next();
        (start_range, _) = get_range_params(&range, file_size).unwrap_or((start_range, end_range));
        end_range = min(start_range + buffer, file_size) - 1;

        let byte_count = end_range - start_range + 1;
       
        return (
            String::from("206 Partial Content"), 
            vec![
                (String::from("Content-type"), format!("video/{}", file::get_extension(&file_path))), // TODO: fix mime
                (String::from("Accept-Ranges"), String::from("bytes")),
                (String::from("Content-Range"), format!("bytes {start_range}-{end_range}/{file_size}")),
                (String::from("Content-Length"), format!("{}", byte_count)),

                // (String::from("Content-Disposition"), format!("inline; filename=\"{}\"", file::get_file_name(&file_path))),
                //(String::from("Content-type"), String::from("application/octet-stream")), // TODO: fix mime
            ], 
            None,
            Some(file::read_range(&file_path, start_range, byte_count).unwrap()),
        );
    }
    // Text content
    return (
        // http://127.0.0.1:9200/oms
        String::from("200 OK"), 
        vec![
            (String::from("Content-type"), file::get_mimetype(path).to_string()),
        ], 
        Some(Box::new(file::read_lines(&file_path).map(|l| l.unwrap_or_default()))),
        None
    );
}

// https://docs.rs/warp-range/latest/src/warp_range/lib.rs.html#1-148
fn get_range_params(range: &Option<&String>, size: u64)->Result<(u64, u64), io::Error> {
    match range {
        Some(range) => {
            let range: Vec<String> = range
                .replace("Range:", "")
                .replace("bytes=", "")
                .trim()
                .split("-")
                .filter_map(|n| if n.len() > 0 {Some(n.to_string())} else {None})
                .collect();
            let start = if range.len() > 0 { 
                range[0].parse::<u64>().unwrap_or_default()
            } else { 
                0 
            };
            let end = if range.len() > 1 {
                range[1].parse::<u64>().unwrap_or_default()
            } else {
                size - 1 
            };
            Ok((start, end))
        },
        None => Ok((0, size - 1))
    }
}