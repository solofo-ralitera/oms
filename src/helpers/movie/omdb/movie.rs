use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct OMDbMovie {
    pub Title: String,
    pub Year: String,
    pub Rated: String,
    pub Released: String,
    pub Runtime: String,
    pub Genre: String,
    pub Director: String,
    pub Writer: String,
    pub Actors: String,
    pub Plot: String,
    pub Language: String,
    pub Country: String,
    pub Awards: String,
    pub Poster: String,
    pub Metascore: String,
    pub imdbRating: String,
    pub imdbVotes: String,
    pub imdbID: String,
    pub Type: String,
}
