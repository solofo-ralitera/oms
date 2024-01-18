use lopdf::Document;

pub struct PdfContent {
    current_page: usize,
    page_number: usize,
    document: Document,
}

///
/// https://ahmadrosid.com/blog/extract-text-from-pdf-in-rust
/// 
impl PdfContent {
    pub fn new(file_path: &str) -> Self {
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
