use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TMDbMovie {
    pub page: usize,
    pub results: Vec<TMDbMovieItem>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TMDbMovieItem {
    pub adult: bool,
    pub backdrop_path: String,
    pub genre_ids: Vec<usize>,
    pub id: usize,
    pub original_language: String,
    pub original_title: String,
    pub overview: String,
    pub popularity: f32,
    pub poster_path: String,
    pub release_date: String, // Date?
    pub title: String,
    pub video: bool,
    pub vote_average: f32,
    pub vote_count: usize,
}
