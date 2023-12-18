pub mod tmdb;
pub mod omdb;

use core::fmt;
use std::{ops::Deref, process::{Command, Stdio}, io::{BufReader, BufRead}};
use crate::helpers::{self, file::remove_extension};
use colored::Colorize;
use regex::Regex;
use serde::{Deserialize, Serialize};


///
/// cargo run -- info --provider=tmdb --cache-path="/media/solofo/r256/v/.oms" "/media/solofo/r256/v"
/// 
pub struct MovieTitle {
    pub title: String,
    pub year: String,
    pub language: String,
    pub adult: bool,
}

fn format_title_remove_point(title: &str) -> String {
    let re_space = Regex::new(r"([^\.]{2,})(\.)").unwrap();
    let title = re_space.replace_all(&title, "${1} ").deref().to_string();

    let re_point = Regex::new(r"(\.)([^.]{2,})").unwrap();
    let title = re_point.replace_all(&title, "${1} ${2}").deref().to_string();
    return title.trim().to_string();
}

pub fn format_title(raw_title: &String) -> MovieTitle {
    let re_year = Regex::new(r"^(.{1,})[\.\(]([0-9]{4})(.{0,})\.").unwrap();
    if let Some((_, [title, year, _])) = re_year.captures(&raw_title).map(|c| c.extract()) {
        let title = format_title_remove_point(title);

        return MovieTitle { 
            title: title, 
            year: year.to_string(),
            language: "en-US".to_string().clone(),
            adult: false,
        };
    }

    let title: String = remove_extension(raw_title);
    let title = format_title_remove_point(&title);

    return MovieTitle { 
        title: title, 
        year: String::new(),
        language: String::new(),
        adult: false,
    };
}

//  TODO: refactor into helpers command
//  ffmpeg -i "input.avi" -c:a copy -c:v vp9 -b:v 100K "input.vp9.mp4"
//  ffmpeg -i new\ romance.AVI new\ romance.mp4
pub fn avi_to_mp4(file_path: &String, dest_path: &String) {
    let mut cmd = Command::new("ffmpeg")
        .args(["-i", file_path, dest_path])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    {
        let stdout = cmd.stdout.as_mut().unwrap();
        let stdout_reader = BufReader::new(stdout);
        let stdout_lines = stdout_reader.lines();

        for line in stdout_lines {
            println!("Read: {:?}", line);
        }
    }
    cmd.wait().unwrap();
}


#[derive(Debug, Deserialize, Serialize)]
pub struct MovieResult {
    pub title: String,
    pub summary: String,
    pub date: String,
    pub genres: Vec<String>,
    pub casts: Vec<String>,
    pub thumb_url: String,
    pub thumb: String,
    pub poster_url: String,    
    pub rating: f32,

    pub provider: String,
    pub provider_id: String,

    pub file_path: String,
    pub file_type: String,
    pub file_hash: String,
}

impl fmt::Display for MovieResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut str = String::new();
        str.push_str(&format!("Title: {} ({})\n\n", self.title.bold(), self.date));

        str.push_str(&helpers::output::draw_image(&self.thumb, (50, 50)));
        str.push_str(&format!("{}\n", self.poster_url));
        
        str.push_str(&format!("\n{}\n", self.summary));
        str.push_str(&format!("\nGenre: {}\n", self.genres.join(", ")));
        str.push_str(&format!("\nCast: {}\n", self.casts.join(", ")));
        write!(f, "{str}")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn format_title_1() {
        let content = String::from("10.Jours.Encore.Sans.Maman.2023.FRENCH");
        let format_title = format_title(&content);

        assert_eq!("10 Jours Encore Sans Maman", format_title.title);
        assert_eq!("2023", format_title.year);
        assert_eq!("en-US", format_title.language);
    }

    #[test]
    fn format_title_2() {
        let content = String::from("Sniper.G.R.I.T.Global.Response.and.Intelligence.Team.2023.FRENCH");
        let format_title = format_title(&content);

        assert_eq!("Sniper G.R.I.T. Global Response and Intelligence Team", format_title.title);
        assert_eq!("2023", format_title.year);
        assert_eq!("en-US", format_title.language);
    }

    #[test]
    fn format_title_3() {
        let content = String::from("The.Equalizer.3.2023.TRUEFRENCH.WEBRip.x264-ONLYMOViE.mkv");
        let format_title = format_title(&content);

        assert_eq!("The Equalizer 3", format_title.title);
        assert_eq!("2023", format_title.year);
        assert_eq!("en-US", format_title.language);
    }

    #[test]
    fn format_title_4() {
        let content = String::from("American.Pie.3.Marions.les (2003).avi");
        let format_title = format_title(&content);

        assert_eq!("American Pie 3. Marions les", format_title.title);
        assert_eq!("2003", format_title.year);
        assert_eq!("en-US", format_title.language);
    }

    #[test]
    fn format_title_5() {
        let content = String::from("au dela de l'illusion.AVI");
        let format_title = format_title(&content);

        assert_eq!("au dela de l'illusion", format_title.title);
        assert!(format_title.year.is_empty());
        assert!(format_title.language.is_empty());
    }

    #[test]
    fn format_title_6() {
        let content = String::from("10 000 BC.avi");
        let format_title = format_title(&content);

        assert_eq!("10 000 BC", format_title.title);
        assert!(format_title.year.is_empty());
        assert!(format_title.language.is_empty());
    }

    #[test]
    fn format_title_7() {
        let content = String::from("71.2014.avi");
        let format_title = format_title(&content);

        assert_eq!("71", format_title.title);
        assert_eq!("2014", format_title.year);
        assert_eq!("en-US", format_title.language);
    }

    #[test]
    fn format_title_8() {
        let content = String::from("1944 (2015).mkv");
        let format_title = format_title(&content);

        assert_eq!("1944", format_title.title);
        assert_eq!("2015", format_title.year);
        assert_eq!("en-US", format_title.language);
    }
    
    #[test]
    fn format_title_9() {
        let content = String::from("Chambre 1408.avi");
        let format_title = format_title(&content);

        assert_eq!("Chambre 1408", format_title.title);
        assert!(format_title.year.is_empty());
        assert!(format_title.language.is_empty());
    }
    
}
