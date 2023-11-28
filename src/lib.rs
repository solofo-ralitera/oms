//! # OMS (may be Open Multitool or Open Media Server)
//!
//! `oms` is a set of utilities intended to make certain common needs more practical.
//! 
//! ## Features
//! 
//! * [x] Help documentation
//! * [ ] Search for a term in a file or directory or an external source
//!     * [x] text file
//!     * [ ] pdf
//!     * [ ] .docx
//!     * [ ] .xlsx
//!     * [ ] link (?)
//! * [ ] Read (output) content of file or an external source
//!     * [x] text file
//!     * [ ] pdf (extract content)
//!     * [ ] .docx
//!     * [ ] .xlsx
//!     * [ ] link (like download media from youtube link)
//! * [ ] Information about any kind of media file (images, movies...)
//!     * [ ] movies (may by movie description from IMDb or other provider)
//!     * [ ] Images (may be image description from IA)
//!     * [ ] link
//! * [ ] Media server (like Universal Media Server)
//!     * [ ] processes all media files contained in a directory
//!     * [ ] provided a web application to search and play media files

pub mod helpers;
pub mod app;
