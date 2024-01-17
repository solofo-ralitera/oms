use super::{result::VideoResult, title::VideoTitle, metadata::VideoMetadata};

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

        let metadata = VideoMetadata::from(video_param.file_path);
    
        result.push(VideoResult {
            title: metadata.title,
            summary: metadata.summary,
            year: metadata.year,
            casts: metadata.casts,
            genres: metadata.genres,

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
