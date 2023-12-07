pub mod tmdb;
pub mod omdb;

use core::fmt;
use crate::helpers::output::draw_image;


pub struct MovieTitle {
    title: String,
    year: String,
    language: String,
    adult: bool,
}

pub fn format_title(title: &String) -> MovieTitle {
    return MovieTitle { 
        title: title.clone(), 
        year: "2023".to_string().clone(),
        language: "".to_string().clone(),
        adult: false,
    };
}


pub struct MovieResult {
    pub title: String,
    pub summary: String,
    pub date: String,
    pub genres: Vec<String>,
    pub casts: Vec<(String, String)>,
    pub thumb_url: String,
    pub thumb: String,
}

impl fmt::Display for MovieResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut str = String::new();
        str.push_str(&format!("\n\n----------------------------------------\n"));

        str.push_str(&format!("Title: {} ({})\n\n", self.title, self.date));

        str.push_str(&draw_image(&self.thumb, (75, 75)));
        str.push_str(&format!("{}\n", self.thumb_url));
        
        str.push_str(&format!("\n{}\n", self.summary));

        str.push_str(&format!("\nGenre: {}\n", self.genres.join(", ")));

        str.push_str(&format!("\nCast\n"));
        for (name, character) in &self.casts {
            str.push_str(&format!("{name} / {character}\n"));
        }
        write!(f, "{str}")
    }
}
