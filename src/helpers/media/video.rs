pub mod metadata;
pub mod result;
pub mod title;

use std::{io, fs};
use crate::helpers::{file, command};
use regex::Regex;
pub mod provider;

///
/// cargo run -- info --cache-path="/media/solofo/MEDIA/.oms" "/media/solofo/MEDIA/films/"
/// 
pub fn transcode(file_path: &String, dest_path: Option<&String>, output: &String) -> Result<Option<String>, io::Error> {
    if !file::is_video_file(file_path) {
        return Err(io::Error::new(
            io::ErrorKind::WriteZero, 
            format!("Video transcode: unsupported extension for {file_path}")
        ))
    }
    if !file::VIDEO_EXTENSIONS.contains(&output.as_str()) {
        return Err(io::Error::new(
            io::ErrorKind::WriteZero, 
            format!("Video transcode: unknown extension {output}")
        ))
    }

    let dest_path = match dest_path {
        None => {
            let re = Regex::new(r"(?i)\.[0-9a-z]{2,}$").unwrap();
            let output = if output.eq("av1") {
                String::from("mkv")
            } else {
                output.to_string()
            };
            re.replace(file_path.as_str(), format!(".{output}")).to_string()
        },
        Some(d) => d.to_string(),
    };

    if file_path.eq(&dest_path) {
        return Ok(None);
    }
    
    if let Ok(_) = file::check_file(&dest_path) {
        return Ok(None);
    }

    println!("Transcoding start {file_path} -> {output}");
    if output.eq("av1") {
        command::exec("ffmpeg",["-i", file_path, "-c:v", "libaom-av1", "-crf", "31", &dest_path]);
    } else {
        command::exec("ffmpeg",["-i", file_path, &dest_path]);
    }

    return match fs::metadata(&dest_path) {
        Ok(metadata) if metadata.is_file() && metadata.len() > 0 => {
            Ok(Some(dest_path))
        },
        _ => Err(io::Error::new(
            io::ErrorKind::WriteZero, 
            format!("Video transcode: {dest_path} not created")
        )),
    };
}

pub fn video_duration(file_path: &String) -> usize {
    if !file::is_video_file(file_path) {
        return 0;
    }
    
    let output = command::exec(
        "ffprobe",
        ["-v", "error", "-show_entries", "format=duration", "-of", "default=noprint_wrappers=1:nokey=1", file_path]        
    );
    let mut size = output.parse::<f64>().unwrap_or(0.).ceil() as usize;
    if size == 0 {
        // Try stream option if the first one failed
        let output = command::exec(
            "ffprobe",
            ["-v", "error", "-select_streams", "v:0", "-show_entries", "stream=duration", "-of", "default=noprint_wrappers=1:nokey=1", file_path]        
        );
        size = output.parse::<f64>().unwrap_or(0.).ceil() as usize;
    }
    return size;
}


/// Search if file with .mp4 extension existe in the same directory,
/// if found use this mp4 file for next process
/// TODO: live re-encoding for other format than mp4 or ts
/// https://www.reddit.com/r/rust/comments/iplph5/encoding_decoding_video_streams_in_rust/
pub fn get_video_file(file_path: &String) -> String {
    /*
    if !file_path.ends_with(".mp4") {
        let re = Regex::new(r"(?i)\.[0-9a-z]{2,}$").unwrap();
        let mp4_file_path = re.replace(file_path.as_str(), ".mp4").to_string();
        if let Ok(f) = file::check_file(&mp4_file_path) {
            return f.to_string();
        }
    }
    */
    return file_path.clone();
}

/// size: in format width:height, e.g. 600:300, 300:-1 (-1 to keep ratio)
/// at: pick image at x% of video duration, and resize to size
pub fn generate_thumb(src_path: &String, dest_path: &String, size: &str, at: f32) -> Vec<u8> {
    let src_path = get_video_file(src_path);

    // Format duration (s) to hh:mm:ss, :0>2 to keep the leading 0
    let duration = (video_duration(&src_path) as f32 * at).ceil() as usize;
    let duration = format!("{:0>2}:{:0>2}:{:0>2}", (duration / 60) / 60, (duration / 60) % 60, duration % 60);

    // ffmpeg need extenstion in output
    let dest_with_extension = format!("{dest_path}.jpeg");

    command::exec(
        "ffmpeg",
        ["-ss", &duration, "-i", &src_path, "-vf", &format!("scale={size}"), "-frames:v", "1", &dest_with_extension]
    );

    return match fs::read(&dest_with_extension) {
        Ok(content) => {
            // Remove output extension in final cache file
            let _ = fs::rename(dest_with_extension, dest_path);
            content
        },
        Err(_) => b"".to_vec(),
    };
}


#[cfg(test)]
mod test {
    use test::title::VideoTitle;

    use super::*;

    #[test]
    fn format_title_1() {
        let content = String::from("10.AAAAA.BBBBB.1111.CC");
        let format_title = VideoTitle::from(&content);

        assert_eq!("10 AAAAA BBBBB", format_title.title);
        assert_eq!(1111, format_title.year);
        assert_eq!("en-US", format_title.language);
    }

    #[test]
    fn format_title_2() {
        let content = String::from("A.B.C.D.EEEE.1111.XXXXXX");
        let format_title = VideoTitle::from(&content);

        assert_eq!("A.B.C.D. EEEE", format_title.title);
        assert_eq!(1111, format_title.year);
        assert_eq!("en-US", format_title.language);
    }

    #[test]
    fn format_title_3() {
        let content_0 = String::from("Aaa.Bbbbbbbb.1.1111.TTTTT.eee");
        let format_title_0 = VideoTitle::from(&content_0);

        assert_eq!("Aaa Bbbbbbbb 1", format_title_0.title);
        assert_eq!(1111, format_title_0.year);
        assert_eq!("en-US", format_title_0.language);

        let content_1 = String::from("Aaa.Bbbbbbbb.1.1111.TTTTT");
        let format_title_1 = VideoTitle::from(&content_1);

        assert_eq!("Aaa Bbbbbbbb 1", format_title_1.title);
        assert_eq!(1111, format_title_1.year);
        assert_eq!("en-US", format_title_1.language);
    }

    #[test]
    fn format_title_4() {
        let content_0 = String::from("Aaa.Bbbbbbbb.1.Cccccc.ddd (1111).eee");
        let format_title_0 = VideoTitle::from(&content_0);

        assert_eq!("Aaa Bbbbbbbb 1. Cccccc ddd", format_title_0.title);
        assert_eq!(1111, format_title_0.year);
        assert_eq!("en-US", format_title_0.language);

        let content_1 = String::from("Aaa.Bbbbbbbb.1.Cccccc.ddd (1111)");
        let format_title_1 = VideoTitle::from(&content_1);

        assert_eq!("Aaa Bbbbbbbb 1. Cccccc ddd", format_title_1.title);
        assert_eq!(1111, format_title_1.year);
        assert_eq!("en-US", format_title_1.language);
    }

    #[test]
    fn format_title_5() {
        let content = String::from("aaa zzzz ee rrrrrrr.AAA");
        let format_title = VideoTitle::from(&content);

        assert_eq!("aaa zzzz ee rrrrrrr", format_title.title);
        assert_eq!(0, format_title.year);
        assert!(format_title.language.is_empty());
    }

    #[test]
    fn format_title_6() {
        let content_0 = String::from("00 000 AA.bbb");
        let format_title_0 = VideoTitle::from(&content_0);

        assert_eq!("00 000 AA", format_title_0.title);
        assert_eq!(0, format_title_0.year);
        assert!(format_title_0.language.is_empty());

        let content_1 = String::from("00 000 AA");
        let format_title_1 = VideoTitle::from(&content_1);

        assert_eq!("00 000 AA", format_title_1.title);
        assert_eq!(0, format_title_1.year);
        assert!(format_title_1.language.is_empty());
    }

    #[test]
    fn format_title_7() {
        let content_0 = String::from("12.3456.avi");
        let format_title_0 = VideoTitle::from(&content_0);

        assert_eq!("12", format_title_0.title);
        assert_eq!(3456, format_title_0.year);
        assert_eq!("en-US", format_title_0.language);

        let content_1 = String::from("12.3456");
        let format_title_1 = VideoTitle::from(&content_1);

        assert_eq!("12", format_title_1.title);
        assert_eq!(3456, format_title_1.year);
        assert_eq!("en-US", format_title_1.language);
    }

    #[test]
    fn format_title_8() {
        let content_0 = String::from("1234 (5678).aaa");
        let format_title_0 = VideoTitle::from(&content_0);

        assert_eq!("1234", format_title_0.title);
        assert_eq!(5678, format_title_0.year);
        assert_eq!("en-US", format_title_0.language);

        let content_0 = String::from("1234 (5678)");
        let format_title_0 = VideoTitle::from(&content_0);

        assert_eq!("1234", format_title_0.title);
        assert_eq!(5678, format_title_0.year);
        assert_eq!("en-US", format_title_0.language);
    }
    
    #[test]
    fn format_title_9() {
        let content_0 = String::from("Azerty 1234.z");
        let format_title_0 = VideoTitle::from(&content_0);

        assert_eq!("Azerty 1234", format_title_0.title);
        assert_eq!(0, format_title_0.year);
        assert!(format_title_0.language.is_empty());

        let content_1 = String::from("Azerty 1234");
        let format_title_1 = VideoTitle::from(&content_1);

        assert_eq!("Azerty 1234", format_title_0.title);
        assert_eq!(0, format_title_1.year);
        assert!(format_title_1.language.is_empty());
    }
    
}
