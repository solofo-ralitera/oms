mod movie;

use std::env;
use std::io::{Error, ErrorKind};
use crate::helpers::http::{self, get_image};
use self::movie::OMDbMovie;

use super::{MovieTitle, MovieResult};

/// 
/// // Search by title/year
/// https://www.omdbapi.com/?t=Murder%20mystery&apikey=5ca3e81d&plot=short&y=2019
/// 
///  cargo run -- info --provider=omdb "Minority report"
/// 
pub struct OMDb {
}

impl OMDb {
    pub fn get_token() -> Result<String, Error> {
        let access_token = env::var("OMDB_KEY").unwrap_or_default();
        if access_token.is_empty() {
            return Err(Error::new(
                ErrorKind::InvalidInput, 
                format!("The envorinment variable OMDB_KEY is not set. You can get api key here https://www.omdbapi.com/apikey.aspx")
            ));
        }
        Ok(access_token)
    }
    
    pub fn info(param: MovieTitle) -> Result<Vec<MovieResult>, Error> {
        let access_token = Self::get_token()?;

        let request_url = format!("https://www.omdbapi.com/");

        let mut params = vec![];
        params.push(("apikey".to_string(), access_token));

        if !param.title.is_empty() {
            params.push(("t".to_string(), param.title));
        }
        if !param.year.is_empty() {
            params.push(("y".to_string(), param.year));
        }

        if let Ok(result) = http::get::<OMDbMovie>(&request_url, vec![], params, true) {
            return Ok(Self::to_movie_result(&result));
        }
        return Err(Error::new(
            ErrorKind::NotConnected, 
            format!("Unable to get information from OMDb")
        ));        
    }

    pub fn to_movie_result(movie: &OMDbMovie) -> Vec<MovieResult> {
        let mut results = vec![];
        results.push(MovieResult {
            title: movie.Title.clone(),
            summary: movie.Plot.clone(),
            date: movie.Year.clone(),
            thumb_url: movie.Poster.clone(),
            thumb: get_image(&format!("{}", movie.Poster)).unwrap_or_default(),
            genres: movie.Genre.split(",").map(|i| i.trim().to_string()).collect(),
            casts: movie.Actors.split(",").map(|i| (i.trim().to_string(), String::new())).collect(),
        });
        return results;
    }
}