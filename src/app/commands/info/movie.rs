use std::sync::mpsc::Sender;
use crate::helpers::movie::{tmdb::TMDb, format_title};
use super::option::InfoOption;


// 
// cargo run -- info /home/solofo/Videos
// cargo run -- info --provider=tmdb "Man on fire"
//
// themoviedb.org (rasta_popolos@yahoo.fr/S.H.Th.)
//  API key: 60537428850ae10b975a375ca66f837e
//  API read access token
//
 // https://developer.themoviedb.org/reference/search-movie
 //
pub struct MovieInfo<'a> {
    pub file_path: &'a String,
    pub info_option: &'a InfoOption,
}

impl<'a> MovieInfo<'a> {
    pub fn info(&self, tx: Sender<String>) {
        let movie_title = format_title(&self.file_path);

        match self.info_option.provider.as_str() {
            "local" => {
                tx.send(format!("WIP")).unwrap_or_default();
            },
            "tmdb" => {
                if let Ok(movies) = TMDb::info(movie_title) {
                    for movie in movies {
                        tx.send(format!("{movie}")).unwrap_or_default();
                    }
                }
            },
            _ => (),
        }
    }
}
