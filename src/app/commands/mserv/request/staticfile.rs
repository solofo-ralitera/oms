use std::collections::HashMap;
use image::EncodableLayout;
use once_cell::sync::Lazy;

use crate::{app::commands::mserv::option::MservOption, helpers::{string, file}};


static STATIC_RESOURCES: Lazy<HashMap<&str, (&str, &[u8])>> = Lazy::new(|| {
    let mut static_resources: HashMap<&str, (&str, &[u8])> = HashMap::new();
    static_resources.insert("/", ("text/html; charset=utf-8", include_bytes!("../resources/assets/index.html")));
    static_resources.insert("/favicon.ico", ("image/x-icon", include_bytes!("../resources/assets/favicon.ico")));
    static_resources.insert("/assets/img/audio.png", ("image/png", include_bytes!("../resources/assets/img/audio.png")));
    static_resources.insert("/assets/js/main.js", ("text/javascript", include_bytes!("../resources/assets/js/main.js")));
    
    static_resources.insert("/assets/js/components/media.js", ("text/javascript", include_bytes!("../resources/assets/js/components/media.js")));
    static_resources.insert("/assets/js/components/medias.js", ("text/javascript", include_bytes!("../resources/assets/js/components/medias.js")));
    
    static_resources.insert("/assets/js/components/player.js", ("text/javascript", include_bytes!("../resources/assets/js/components/player.js")));
    static_resources.insert("/assets/js/components/player/video.js", ("text/javascript", include_bytes!("../resources/assets/js/components/player/video.js")));
    static_resources.insert("/assets/js/components/player/audio.js", ("text/javascript", include_bytes!("../resources/assets/js/components/player/audio.js")));

    static_resources.insert("/assets/js/components/search.js", ("text/javascript", include_bytes!("../resources/assets/js/components/search.js")));
    static_resources.insert("/assets/js/components/summary.js", ("text/javascript", include_bytes!("../resources/assets/js/components/summary.js")));

    static_resources.insert("/assets/js/components/config.js", ("text/javascript", include_bytes!("../resources/assets/js/components/config.js")));
    static_resources.insert("/assets/js/components/config/scandir.js", ("text/javascript", include_bytes!("../resources/assets/js/components/config/scandir.js")));
    static_resources.insert("/assets/js/components/config/summary.js", ("text/javascript", include_bytes!("../resources/assets/js/components/config/summary.js")));
    static_resources.insert("/assets/js/components/config/genres.js", ("text/javascript", include_bytes!("../resources/assets/js/components/config/genres.js")));

    static_resources.insert("/assets/js/services/app.js", ("text/javascript", include_bytes!("../resources/assets/js/services/app.js")));
    static_resources.insert("/assets/js/services/elastic.js", ("text/javascript", include_bytes!("../resources/assets/js/services/elastic.js")));
    static_resources.insert("/assets/js/services/EventBus.js", ("text/javascript", include_bytes!("../resources/assets/js/services/EventBus.js")));
    static_resources.insert("/assets/js/services/history.js", ("text/javascript", include_bytes!("../resources/assets/js/services/history.js")));

    return static_resources;
});

pub fn process(path: &str, _: &Vec<String>, serv_option: &MservOption) -> Option<(String, Vec<(String, String)>, Option<Box<dyn Iterator<Item = String>>>, Option<Vec<u8>>)> {
    match STATIC_RESOURCES.get(path) {
        None => None,
        Some((content_type, content)) => return Some((
            String::from("200 OK"), 
            vec![
                (String::from("Content-type"), content_type.to_string()),
            ], 
            None,
            if path.ends_with(".js") {
                let mut content = string::bytes_replace(content, b"\"BASE_URL\"", format!("\"{}\"", serv_option.base_path).as_bytes());
                content = string::bytes_replace(content.as_bytes(), b"\"TRANSCODE_OUTPUT\"", format!("\"{}\"", serv_option.transcode_output).as_bytes());
                content = string::bytes_replace(content.as_bytes(), b"\"TRANSCODE_THREAD\"", format!("{}", serv_option.transcode_thread).as_bytes());
                content = string::bytes_replace(content.as_bytes(), b"[\"VIDEO_EXTENSIONS\"]", serde_json::to_string(&file::VIDEO_EXTENSIONS).unwrap_or(String::new()).as_bytes());
                match serv_option.elastic.as_ref() {
                    Some(elastic) => {
                        content = string::bytes_replace(content.as_bytes(), b"\"ELASTIC_URL\"", format!("\"{}\"", elastic.url).as_bytes());
                    },
                    _ => (),
                };
                Some(content)
            } else {
                Some(content.to_vec())
            }
        ))
    }
}
