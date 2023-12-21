use std::sync::mpsc::Sender;
use super::option::SearchOption;

///
/// cargo run -- search --cache-path="/media/solofo/MEDIA/.oms" "/media/solofo/MEDIA/films/" fire
/// 
pub struct MovieSearch<'a> {
    pub file_path: &'a String,
    pub search_term: &'a String,
    pub search_option: &'a SearchOption,   
}

impl<'a> MovieSearch<'a> {
    pub fn search(&self, _: Sender<String>) {
        println!("{}", self.file_path);
    }
}