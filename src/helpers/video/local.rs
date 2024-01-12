use super::{VideoTitle, VideoResult};

type Result<T> = std::result::Result<T, std::io::Error>;

pub struct LocalParam<'a> {
    pub video_title: &'a VideoTitle,
    pub raw_title: &'a String,
    pub file_path: &'a String,
    pub base_path: &'a String,
}
pub struct Local {
}

impl Local {
    pub fn info(video_param: LocalParam) -> Result<Vec<VideoResult>> {
        let mut result = vec![] ;
        
        result.push(VideoResult {
            title: video_param.video_title.title.clone(),
            summary: sumarize(&video_param),
            year: video_param.video_title.year.clone(),
            casts: vec![],
            genres: vec![],
            thumb_url: String::new(),
            poster_url: String::new(),
            rating: 1.,
            
            thumb: String::new(), 

            provider: String::from("local"),
            provider_id: String::new(),


            file_path: String::new(),
            file_type: String::from("video"),
            hash: String::new(),
            modification_time: 0,
            duration: 0,
            file_size: 0,
        });

        return Ok(result);
    }
}

fn sumarize(video_param: &LocalParam) -> String {
    return format!(
        "{} {} {} {}", 
        video_param.video_title.title, 
        video_param.video_title.year,
        video_param.raw_title,
        video_param.file_path,
    );
}