use std::{sync::mpsc::Sender, time::SystemTime};
use chrono::{DateTime, Utc};
use colored::Colorize;
use crate::helpers::{cache, db::elastic::Elastic, media::video::{self, result::get_video_result}};
use super::option::InfoOption;

/// 
/// cargo run -- info /home/solofo/Videos
/// cargo run -- info --provider=tmdb "Man on fire"
/// cargo run -- info --provider=omdb --cache-path="/media/solofo/MEDIA/.oms" "/media/solofo/MEDIA/films/"
///
/// https://developer.themoviedb.org/reference/search-movie
///
pub struct VideoInfo<'a> {
    pub video_raw_name: &'a String,
    pub file_path: &'a String,
    pub info_option: &'a InfoOption,
}

impl<'a> VideoInfo<'a> {
    pub fn info(&self, tx: Sender<String>) {
        match get_video_result(&self.video_raw_name, &self.file_path, &self.info_option.base_path, &self.info_option.provider) {
            Ok(mut videos) => {
                save_elastic(&mut videos, &self.info_option.elastic);
                for video in videos {
                    tx.send(format!("\
\n------------------------------------------------------------------------
{video}\n")).unwrap_or_default();
                }
            },
            Err(err) => {
                log_error(&self);
                if self.info_option.display_preview == false {
                    println!("\n{}\n", err.to_string().on_red());
                } else {
                    return tx.send(format!("\n{}\n", err.to_string().on_red())).unwrap_or_default();
                }
            }
        }
    }
}

fn save_elastic(videos: &mut Vec<video::result::VideoResult>, elastic: &Option<Elastic>) {
    if let Some(el) = elastic {
        // Get and save only first result
        if let Some(video) = videos.iter_mut().next() {
            el.insert(&video.hash, &video);
        }
    }
}

fn log_error(video: &VideoInfo) {
    let curr_time = SystemTime::now();
    let dt: DateTime<Utc> = curr_time.into();

    let content = format!("{}\n", video.file_path);
    cache::append_cache_content(&dt.format("%Y-%m-%d").to_string(), &content, ".http-error");
}
