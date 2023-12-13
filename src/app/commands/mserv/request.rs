use bytes::Bytes;
use crate::helpers::file;


///
/// Return: status: 200 OK, headers, content
/// 
pub fn process(path: &str, verb: &str) -> (String, Vec<(String, String)>, Option<Box<dyn Iterator<Item = String>>>, Option<Bytes>){
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
    if !path.starts_with("/assets/") {
        return (String::from("404 Not Found"), vec![], None, None);
    }
    let file_path = format!("./resources/mserv{path}");
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
    }
    // Text content
    return (
        String::from("200 OK"), 
        vec![
            (String::from("Content-type"), file::get_mimetype(path).to_string()),
        ], 
        Some(Box::new(file::read_lines(&file_path).map(|l| l.unwrap_or_default()))),
        None
    );
}
