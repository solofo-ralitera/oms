pub mod content;
pub mod result;
mod metadata;
mod provider;

use std::{io, fs};
use sha256::digest;

use crate::helpers::{file, rtrim_char, ltrim_char, command, media::pdf::result::PdfResult};

use self::provider::{exif, pdfprov, local};

use super::normalize_media_title;

pub fn get_pdf_result(base_path: &String, file_path: &String) -> Result<PdfResult, io::Error> {
    let file_size: usize = file::file_size(file_path).unwrap_or_default() as usize;
    let relative_file_path = file_path.replace(base_path, "");

    let hash = file::sha256(file_path).unwrap_or(digest(&relative_file_path));

    let modification_time = file::get_creation_time(file_path);

    let metadata = if let Some(r) = exif::from_exif(file_path) {
        r
    } else if let Some(r) = pdfprov::from_pdf(file_path) {
        r
    } else {
        local::from_local(file_path).unwrap()
    };

    return Ok(PdfResult {
        title: normalize_media_title(&metadata.title),
        summary: metadata.summary,
        casts: metadata.casts,
        genres: metadata.genres,
        year: metadata.year,

        provider: String::from("local"),

        rating: 1.,
        file_type: String::from("pdf"),
        file_path: relative_file_path,
        full_path: file_path.to_string(),
        hash: hash,
        modification_time: modification_time,
        duration: 0,
        file_size: file_size,
    });    
}

// TODO: generate thumb
pub fn generate_thumb(src_path: &String, dest_path: &String, size: &str) -> Vec<u8> {
    let size = size.replace(":", "x");
    let size = size.replace("-1", "");
    let size = rtrim_char(&size.to_string(), 'x');
    let size = ltrim_char(&size.to_string(), 'x');

    // Output need extenstion in output
    let dest_with_extension = format!("{dest_path}.jpeg");
    command::exec_stdout(
        "convert",
        ["-thumbnail", &format!("{size}^>"), "-background", "white", "-alpha", "remove", &format!("{src_path}[0]"), &dest_with_extension]    
    );

    return match fs::read(&dest_with_extension) {
        Ok(content) => {
            // Remove output extension in final cache file
            let _ = fs::rename(dest_with_extension, dest_path);
            content
        },
        Err(_) => b"".to_vec(),
    };
}
