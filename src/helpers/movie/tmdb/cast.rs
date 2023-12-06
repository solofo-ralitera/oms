use std::io;
use serde::{Deserialize, Serialize};
use crate::helpers::http;


#[derive(Debug, Deserialize, Serialize)]
pub struct TMDbCast {
    pub id: usize,
    pub cast: Vec<TMDbCastItem>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TMDbCastItem {
    pub id: usize,
    pub name: String,
    pub character: String,
    pub popularity: f32,
}

impl TMDbCast {
    pub fn casts(access_token: &String, movie_id: usize) -> Result<TMDbCast, io::Error> {
        let request_url = format!("https://api.themoviedb.org/3/movie/{movie_id}/credits?language=en-US");
    
        let mut headers = vec![];
        headers.push(("accept".to_string(), "application/json".to_string()));
        headers.push(("Authorization".to_string(), format!("Bearer {}", access_token)));
        
        if let Ok(mut result) = http::get::<TMDbCast>(&request_url, headers, vec![], true) {
            result.cast.sort_by(|c1, c2| c2.popularity.partial_cmp(&c1.popularity).unwrap());
            return Ok(result);
        }
        return Err(io::Error::new(
            io::ErrorKind::NotConnected, 
            format!("Unable to get cast from TMDb")
        ));    
    }
}