use crate::helpers::{media::pdf::metadata::PdfMetadata, file};

pub fn from_local(file_path: &String) -> Option<PdfMetadata> {
    return Some(PdfMetadata {
        title: file::get_file_name(file_path), 
        summary: String::new(),
        year: 0,
        casts: vec![],
        genres: vec![],
    });
}