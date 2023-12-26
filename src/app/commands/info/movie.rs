use std::{sync::mpsc::Sender, time::SystemTime};
use chrono::{DateTime, Utc};
use colored::Colorize;
use crate::helpers::{movie::{self, MovieResult}, cache, db::elastic::Elastic};
use super::option::InfoOption;

/// 
/// cargo run -- info /home/solofo/Videos
/// cargo run -- info --provider=tmdb "Man on fire"
/// cargo run -- info --provider=omdb --cache-path="/media/solofo/MEDIA/.oms" "/media/solofo/MEDIA/films/"
///
/// https://developer.themoviedb.org/reference/search-movie
///
pub struct MovieInfo<'a> {
    pub movie_raw_name: &'a String,
    pub file_path: &'a String,
    pub info_option: &'a InfoOption,
}

impl<'a> MovieInfo<'a> {
    pub fn info(&self, tx: Sender<String>) {
        match movie::get_movie_result(&self.movie_raw_name, &self.file_path, &self.info_option.base_path) {
            Ok(movies) => {
                save_elastic(&movies, &self.info_option.elastic);
                for movie in movies {
                    tx.send(format!("\
\n------------------------------------------------------------------------
{movie}\n")).unwrap_or_default();
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


fn save_elastic(movies: &Vec<MovieResult>, elastic: &Option<Elastic>) {
    if let Some(el) = elastic {
        if let Some(movie) = movies.iter().next() {
            // Save only first result
            el.insert(&movie.hash, &movie);
        }
    }    
}

fn log_error(movie: &MovieInfo) {
    let curr_time = SystemTime::now();
    let dt: DateTime<Utc> = curr_time.into();

    let content = format!("{}\n", movie.file_path);
    cache::append_cache_content(&dt.format("%Y-%m-%d").to_string(), &content, ".http-error");
}
