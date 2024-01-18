use regex::Regex;

use super::{file, ltrim, string};

pub mod audio;
pub mod image;
pub mod pdf;
pub mod video;


pub fn normalize_media_title(title: &String) -> String {
    let mut title = title.replace("_", " ");
    // TODO: better way to safe remove true extensions
    for extension in file::VIDEO_EXTENSIONS {
        title = ltrim(&title, &(".".to_owned() + extension));
    }
    for extension in file::PDF_EXTENSIONS {
        title = ltrim(&title, &(".".to_owned() + extension));
    }
    for extension in file::MS_EXTENSIONS {
        title = ltrim(&title, &(".".to_owned() + extension));
    }
    for extension in file::IMAGE_EXTENSIONS {
        title = ltrim(&title, &(".".to_owned() + extension));
    }
    for extension in file::AUDIO_EXTENSIONS {
        title = ltrim(&title, &(".".to_owned() + extension));
    }

    let re_space = Regex::new(r" {1,}").unwrap();
    title = re_space.replace(&title, " ").to_string();
    title = string::remove_null_char(&title);

    return title;
}