use std::{sync::mpsc::Sender, io, time::SystemTime};
use chrono::{DateTime, Utc};
use colored::Colorize;
use crate::helpers::{movie::{tmdb::TMDb, format_title, omdb::OMDb, MovieResult}, cache, db::{elastic::Elastic, kvstore::KVStore}, file};
use super::option::InfoOption;

type Result<T> = std::result::Result<T, std::io::Error>;

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
    pub fn info(&self, tx: Sender<String>, kv: &mut KVStore) {

        match self.get_movie_result(kv) {
            Ok(movies) => {
                for movie in movies {
                    tx.send(format!("\
\n------------------------------------------------------------------------
{movie}\n")).unwrap_or_default();
                }
            },
            Err(err) => {
                if self.info_option.display_preview == false {
                    println!("\n{}\n", err.to_string().on_red());
                } else {
                    return tx.send(format!("\n{}\n", err.to_string().on_red())).unwrap_or_default();
                }
            }
        }

    }

    fn get_movie_result(&self, kv: &mut KVStore) -> Result<Vec<MovieResult>> {
        let movie_title = format_title(&self.movie_raw_name);
        
        let file_hash = match kv.get(&self.file_path) {
            Some(hash) => hash,
            None => {
                let hash = file::sha256(&self.file_path).unwrap_or_default();
                kv.add(&self.file_path, &hash);
                hash
            },
        };

        // Check cache
        if let Some((_, content)) = cache::get_cache(&file_hash, ".movie") {
            if let Ok(result) = serde_json::from_str::<Vec<MovieResult>>(&content) {
                if result.len() > 0 {
                    // save_data(&file_hash, &mut result, &self);
                    // save_elastic(&result, &self.info_option.elastic);
                    return Ok(result);
                }
            }
        }

        // Find first in tmdb, if not found switch to omdb, otherwise fall in error
        if let Ok(mut result) = TMDb::info(&movie_title) {
            save_data(&file_hash, &mut result, &self);
            save_elastic(&result, &self.info_option.elastic);
            return Ok(result);
        } else if let Ok(mut result) = OMDb::info(&movie_title) {
            save_data(&file_hash, &mut result, &self);
            save_elastic(&result, &self.info_option.elastic);
            return Ok(result);
        } else {
            log_error(&self);
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput, 
                format!("Unable to find information about the movie {}", movie_title.title.on_red())
            ));
        }
    }
}


fn save_elastic(movies: &Vec<MovieResult>, elastic: &Option<Elastic>) {
    if let Some(el) = elastic {
        if let Some(movie) = movies.iter().next() {
            // Save only first result
            el.insert(&movie);
        }
    }    
}

fn save_data(file_hash: &String, movies: &mut Vec<MovieResult>, movie_info: &MovieInfo) {
    if file_hash.is_empty() {
        return;
    }
    cache::write_cache_json(&file_hash, &movies, ".movie");
    for movie in movies {
        movie.file_path = movie_info.file_path.clone();
        movie.file_hash = file_hash.clone();
    }    
}

fn log_error(movie: &MovieInfo) {
    let curr_time = SystemTime::now();
    let dt: DateTime<Utc> = curr_time.into();

    let content = format!("{}\n", movie.file_path);
    cache::append_cache_content(&dt.format("%Y-%m-%d").to_string(), &content, ".http-error");
}
