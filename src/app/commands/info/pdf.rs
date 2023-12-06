use std::sync::mpsc::Sender;

use crate::helpers::pdf::PdfInfo as HelperPdf;

///
/// cargo run -- info /home/solofo/Documents/books/
/// 
pub struct PdfInfo<'a> {
    pub file_path: &'a String,
}

impl<'a> PdfInfo<'a> {
    pub fn info(&self, tx: Sender<String>) {
        if let Ok(pdf_info) = HelperPdf::read(&self.file_path) {
            tx.send(format!("{}", self.file_path)).unwrap_or_default();

            tx.send(format!("Title: {}", pdf_info.title)).unwrap_or_default();
            tx.send(format!("Author: {}", pdf_info.author)).unwrap_or_default();
            tx.send(format!("Creator: {}", pdf_info.creator)).unwrap_or_default();
            tx.send(format!("Keywords: {}", pdf_info.keywords)).unwrap_or_default();
            tx.send(format!("Subject: {}", pdf_info.subject)).unwrap_or_default();

            tx.send(format!("\n")).unwrap_or_default();
        }
    }
}
