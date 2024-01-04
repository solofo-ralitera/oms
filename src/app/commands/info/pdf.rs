use std::sync::mpsc::Sender;

use colored::Colorize;

use crate::helpers::{pdf::{get_pdf_result, PdfResult}, db::elastic::Elastic};

use super::option::InfoOption;

///
/// cargo run -- info /home/solofo/Documents/books/
/// 
pub struct PdfInfo<'a> {
    pub file_path: &'a String,
    pub info_option: &'a InfoOption,
}

impl<'a> PdfInfo<'a> {
    pub fn info(&self, tx: Sender<String>) {
        match get_pdf_result(&self.info_option.base_path, &self.file_path) {
            Ok(pdf) => {
                save_elastic(&pdf, &self.info_option.elastic);
                tx.send(format!("\
\n------------------------------------------------------------------------
{pdf}\n")).unwrap_or_default();
            },
            Err(err) => {
                if self.info_option.display_preview == false {
                    println!("\n{}\n", err.to_string().on_red());
                } else {
                    return tx.send(format!("\n{}\n", err.to_string().on_red())).unwrap_or_default();
                }                
            }
        }
    }
}

fn save_elastic(pdf: &PdfResult, elastic: &Option<Elastic>) {
    if let Some(el) = elastic {
        el.insert(&pdf.hash, &pdf);
    }
}