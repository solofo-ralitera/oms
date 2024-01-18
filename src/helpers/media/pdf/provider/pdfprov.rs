use pdf::file::FileOptions;
use regex::Regex;
use crate::helpers::media::pdf::metadata::PdfMetadata;

pub fn from_pdf(file_path: &str) -> Option<PdfMetadata> {
    if let Ok(file) = FileOptions::cached().open(&file_path) {
        if let Some(ref info) = file.trailer.info_dict {
                /*
                title: info.title.as_ref().map(|p| p.to_string_lossy()).unwrap_or_default(),
                author: info.author.as_ref().map(|p| p.to_string_lossy()).unwrap_or_default(),
                creator: info.creator.as_ref().map(|p| p.to_string_lossy()).unwrap_or_default(),
                keywords: info.keywords.as_ref().map(|p| p.to_string_lossy()).unwrap_or_default(),
                subject: info.subject.as_ref().map(|p| p.to_string_lossy()).unwrap_or_default(),
                content: PdfContent::new(file_path),
                */
            let re_comma = Regex::new(r"[,;/]").unwrap();
            let summary = format!("{}", info.subject.as_ref().map(|p| p.to_string_lossy()).unwrap_or_default());
            let casts: Vec<String> = re_comma.split(&info.author.as_ref().map(|p| p.to_string_lossy()).unwrap_or_default()).into_iter().map(|l| l.trim().to_string()).filter(|l| l.len() > 1).collect();
            let genres: Vec<String> = re_comma.split(&info.keywords.as_ref().map(|p| p.to_string_lossy()).unwrap_or_default()).into_iter().map(|l| l.trim().to_string()).filter(|l| l.len() > 1).collect();

            return Some(PdfMetadata {
                title: info.title.as_ref().map(|p| p.to_string_lossy()).unwrap_or_default(), 
                summary: summary.trim().to_string(),
                year: 0,
                casts: casts,
                genres: genres,
            });                
        }
    }
    None
}
