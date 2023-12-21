//! # OMS (may be Open Multitool or Open Media Server)
//!
//! `oms` is a set of utilities intended to make certain common needs more practical.
//! 
//! ## Features
//! 
//! * [x] Help documentation
//! * [ ] Search for a term in a file or directory or an external source
//!     * [x] text file
//!     * [x] pdf
//!     * [x] .docx
//!     * [x] .xlsx
//!     * [x] .pptx
//!     * [ ] movie
//! * [ ] Read (output) content of file or an external source
//!     * [x] text file
//!     * [ ] pdf (extract content)
//!     * [ ] .docx
//!     * [ ] .xlsx
//!     * [ ] link (like download media from youtube link)
//! * [ ] Information about any kind of media file (images, movies...)
//!     * [x] pdf
//!     * [x] movies
//!         * [x] Info from TMDb
//!         * [x] Info from OMDb
//!     * [ ] Images (may be image description from IA)
//!     * [ ] link
//! * [x] Media server (like Universal Media Server)
//!     * [x] process all media files contained in a directory
//!     * [x] provided a web application to search and play media files

pub mod helpers;
pub mod app;
