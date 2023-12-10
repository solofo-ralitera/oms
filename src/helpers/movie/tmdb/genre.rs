use serde::{Deserialize, Serialize};
use std::{fmt, io::{Error, ErrorKind}};
use crate::helpers::http;

type Result<T> = std::result::Result<T, std::io::Error>;

#[derive(Debug, Deserialize, Serialize)]
pub struct TMDbGenre {
    pub genres: Vec<TMDbGenreItem>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TMDbGenreItem {
    pub id: usize,
    pub name: String,
}

impl fmt::Display for TMDbGenre {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut str = String::new();
        for genre in &self.genres {
            str.push_str(&format!("{}-{}\n", genre.id, genre.name));
        }
        write!(f, "{str}")
    }
}

impl TMDbGenre {
    pub fn genres(access_token: &String) -> Result<TMDbGenre> {
        let request_url = format!("https://api.themoviedb.org/3/genre/movie/list?language=en");
    
        let mut headers = vec![];
        headers.push(("accept".to_string(), "application/json".to_string()));
        headers.push(("Authorization".to_string(), format!("Bearer {}", access_token)));
        
        if let Ok(result) = http::get::<TMDbGenre>(&request_url, headers, vec![], true) {
            return Ok(result);
        }
        return Err(Error::new(
            ErrorKind::NotConnected, 
            format!("Unable to get genres from TMDb")
        )); 
    }
}
