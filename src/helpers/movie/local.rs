use super::{MovieTitle, MovieResult};

type Result<T> = std::result::Result<T, std::io::Error>;

pub struct LocalParam<'a> {
    pub movie_title: &'a MovieTitle,
    pub raw_title: &'a String,
    pub file_path: &'a String,
    pub base_path: &'a String,
}
pub struct Local {
}

impl Local {
    pub fn info(movie_param: LocalParam) -> Result<Vec<MovieResult>> {
        let mut result = vec![] ;
        
        result.push(MovieResult {
            title: movie_param.movie_title.title.clone(),
            summary: sumarize(&movie_param),
            year: movie_param.movie_title.year.clone(),
            casts: vec![],
            genres: vec![],
            thumb_url: format!("/thumb{}", &movie_param.file_path),           
            poster_url: format!("/poster{}", &movie_param.file_path),
            rating: 1.,
            
            thumb: String::new(), 

            provider: String::from("local"),
            provider_id: String::new(),


            file_path: String::new(),
            file_type: String::from("movie"),
            hash: String::new(),
            modification_time: 0,
            duration: 0,
        });

        return Ok(result);
    }
}

fn sumarize(movie_param: &LocalParam) -> String {
    return format!(
        "{} {} {} {}", 
        movie_param.movie_title.title, 
        movie_param.movie_title.year,
        movie_param.raw_title,
        movie_param.file_path,
    );
}