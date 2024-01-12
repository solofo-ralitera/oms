mod movie;

use std::env;
use std::io::{Error, ErrorKind};
use crate::helpers::http::{self, get_image};
use self::movie::OMDbMovie;
use super::{VideoTitle, VideoResult};

type Result<T> = std::result::Result<T, std::io::Error>;


/// 
/// // Search by title/year
/// https://www.omdbapi.com/?t=Murder%20mystery&apikey=5ca3e81d&plot=short&y=2019
/// 
///  cargo run -- info --provider=omdb "Minority report"
/// 
pub struct OMDb {
}

impl OMDb {
    pub fn get_token() -> Result<String> {
        let access_token = env::var("OMDB_KEY").unwrap_or_default();
        if access_token.is_empty() {
            return Err(Error::new(
                ErrorKind::InvalidInput, 
                format!("The envorinment variable OMDB_KEY is not set. You can get api key here https://www.omdbapi.com/apikey.aspx")
            ));
        }
        Ok(access_token)
    }
    
    pub fn info(movie: &VideoTitle) -> Result<Vec<VideoResult>> {
        let access_token = Self::get_token()?;

        let request_url = format!("https://www.omdbapi.com/");

        let mut params = vec![];
        params.push(("apikey".to_string(), access_token));

        if !movie.title.is_empty() {
            params.push(("t".to_string(), movie.title.to_string()));
        }
        if !movie.year.is_empty() {
            params.push(("y".to_string(), movie.year.to_string()));
        }

        match http::get::<OMDbMovie>(&request_url, vec![], params, true) {
            Ok(result) => return Ok(Self::to_video_result(&result)),
            Err(err) => return Err(Error::new(
                ErrorKind::InvalidData, 
                format!("Unable to get information from OMDb: {err}")
            )),
        }
    }

    pub fn to_video_result(movie: &OMDbMovie) -> Vec<VideoResult> {
        let mut results = vec![];
        results.push(VideoResult {
            title: movie.Title.clone(),
            summary: movie.Plot.clone(),
            year: movie.Year.trim().get(0..=3).unwrap_or("").to_string(),
            thumb_url: movie.Poster.clone(),
            thumb: get_image(&format!("{}", movie.Poster)).unwrap_or_default(),
            poster_url: movie.Poster.clone(),            
            genres: movie.Genre.split(",").map(|i| i.trim().to_string()).collect(),
            casts: movie.Actors.split(",").map(|i| i.trim().to_string()).collect(),
            rating: movie.imdbRating.parse().unwrap_or_default(),

            provider: String::from("api"),
            provider_id: movie.imdbID.to_string(),

            file_path: String::new(),
            file_type: String::from("video"),
            hash: String::new(),
            modification_time: 0,
            duration: 0,
            file_size: 0,
        });
        return results;
    }
}