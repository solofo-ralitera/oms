mod movie;
mod genre;
mod cast;

use std::env;
use std::io::{Error, ErrorKind};
use crate::helpers::http::{self, get_image};
use super::{MovieTitle, MovieResult};
use movie::TMDbMovie;
use genre::TMDbGenre;
use cast::TMDbCast;

type Result<T> = std::result::Result<T, std::io::Error>;

///
/// // Get genre list
/// https://developer.themoviedb.org/reference/genre-movie-list
/// 
/// // Get actors
/// https://developers.themoviedb.org/3/movies/get-movie-credits
/// 
/// // Search by title/year
/// https://api.themoviedb.org/3/search/movie?query=The%20Shepherd&include_adult=false&language=fr-FR&primary_release_year=2023&page=1&year=2023
/// 
/// // Detail
/// https://api.themoviedb.org/3/movie/343611?api_key=API_KEY
/// 
///  cargo run -- info --provider=tmdb "Minority report"
/// cargo run -- info --provider=tmdb "Medellin"
/// 
pub struct TMDb {
}

impl TMDb {
    pub fn get_token() -> Result<String> {
        let access_token = env::var("TMDB_ACCESS_TOKEN").unwrap_or_default();
        if access_token.is_empty() {
            return Err(Error::new(
                ErrorKind::InvalidInput, 
                format!("The envorinment variable TMDB_ACCESS_TOKEN is not set. You can get api key here https://developer.themoviedb.org/v4/reference/intro/getting-started")
            ));
        }
        Ok(access_token)
    }
    
    pub fn info(param: &MovieTitle) -> Result<Vec<MovieResult>> {
        let access_token = Self::get_token()?;

        let request_url = format!("https://api.themoviedb.org/3/search/movie");
        let mut params = vec![];
        if !param.title.is_empty() {
            params.push(("query".to_string(), param.title.to_string()));
        }
        if !param.year.is_empty() {
            params.push(("primary_release_year".to_string(), param.year.clone()));
            params.push(("year".to_string(), param.year.clone()));
        }
        if !param.language.is_empty() {
            params.push(("language".to_string(), param.language.to_string()));
        }

        let include_adult = param.adult.to_string();
        let page = "1".to_string();
        params.push(("include_adult".to_string(), include_adult));
        params.push(("page".to_string(), page));


        let mut headers = vec![];
        headers.push(("accept".to_string(), "application/json".to_string()));
        headers.push(("Authorization".to_string(), format!("Bearer {}", access_token)));

        if let Ok(result) = http::get::<TMDbMovie>(&request_url, headers, params, true) {
            return Ok(Self::to_movie_result(&result));
        }
        return Err(Error::new(
            ErrorKind::NotConnected, 
            format!("Unable to get information from TMDb")
        ));        
    }

    pub fn to_movie_result(movies: &TMDbMovie) -> Vec<MovieResult> {
        let access_token = Self::get_token().unwrap_or_default();
        let genres = TMDbGenre::genres(&access_token).unwrap();

        let mut results = vec![];
        for item in &movies.results {
            let casts = TMDbCast::casts(&access_token, item.id).unwrap_or(TMDbCast { 
                id: 0,
                cast: vec![],
            });
            let casts: Vec<String> = casts.cast.iter()
                .filter(|cast| cast.popularity > 10.)
                .map(|cast| cast.name.clone())
                .collect();

            let g = genres.genres.iter()
                .filter(|genre| item.genre_ids.contains(&genre.id))
                .map(|genre| genre.name.clone())
                .collect();
            
            let thumb_url = format!("https://image.tmdb.org/t/p/w300{}", item.backdrop_path);
            let thumb_path = get_image(&thumb_url).unwrap_or_default();

            results.push(MovieResult {
                title: item.title.clone(),
                summary: item.overview.clone(),
                date: item.release_date.clone(),
                thumb_url: thumb_url,
                thumb: thumb_path,
                poster_url: format!("http://image.tmdb.org/t/p/w780{}", item.poster_path),
                genres: g,
                casts: casts,
                rating: item.vote_average,

                provider: String::from("tmdb"),
                provider_id: item.id.to_string(),
                
                file_path: String::new(),
                file_type: String::from("movie"),
                file_hash: String::new(),
            });
        }
        results
    }
}