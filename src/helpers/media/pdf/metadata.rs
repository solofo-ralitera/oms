pub struct PdfMetadata {
    pub title: String, 
    pub summary: String, // Subjet + Description
    pub year: u16, // Date
    pub casts: Vec<String>, // author, split ; or , then remove single or empty char
    pub genres: Vec<String>, // Keywords, split ; or , then remove single or empty char
}
