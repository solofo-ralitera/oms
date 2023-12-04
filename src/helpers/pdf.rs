use pdf::{file::FileOptions, PdfError};
use lopdf::Document;

pub struct PdfInfo {
    pub title: String,
    pub author: String,
    pub creator: String,
    pub keywords: String,
    pub subject: String,
    pub content: PdfContent,
}

impl PdfInfo {
    pub fn read(file_path: &str) -> Result<PdfInfo, PdfError> {
        match FileOptions::cached().open(&file_path) {
            Ok(file) => {

                if let Some(ref info) = file.trailer.info_dict {
                    return Ok(PdfInfo {
                        title: info.title.as_ref().map(|p| p.to_string_lossy()).unwrap_or_default(),
                        author: info.author.as_ref().map(|p| p.to_string_lossy()).unwrap_or_default(),
                        creator: info.creator.as_ref().map(|p| p.to_string_lossy()).unwrap_or_default(),
                        keywords: info.keywords.as_ref().map(|p| p.to_string_lossy()).unwrap_or_default(),
                        subject: info.subject.as_ref().map(|p| p.to_string_lossy()).unwrap_or_default(),
                        content: PdfContent::new(file_path),
                    });
                }
                return Ok(PdfInfo {
                    title: "".to_string(),
                    author: "".to_string(),
                    creator: "".to_string(),
                    keywords: "".to_string(),
                    subject: "".to_string(),
                    content: PdfContent::new(file_path),
                });
            },
            Err(err) => Err(err),
        }
    }
}

pub struct PdfContent {
    current_page: usize,
    page_number: usize,
    document: Document,
}

///
/// https://ahmadrosid.com/blog/extract-text-from-pdf-in-rust
/// 
impl PdfContent {
    fn new(file_path: &str) -> Self {
        if let Ok(document) = Document::load(file_path) {
            return PdfContent {
                current_page: 1,
                page_number: document.get_pages().len(),
                document: document,
            };
        }
        return PdfContent {
            current_page: 1,
            page_number: 0,
            document: Document::new(),
        }
    }
}

impl Iterator for PdfContent {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_page > self.page_number {
            return None;
        }
        let text = self.document.extract_text(&[self.current_page as u32]);
        self.current_page += 1;
        Some(text.unwrap_or_default().replace("?Identity-H Unimplemented?", ""))
    }
}
