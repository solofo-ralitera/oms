use std::sync::mpsc::Sender;
use colored::Colorize;
use crate::helpers::{media::audio::{get_audio_result, AudioResult}, db::elastic::Elastic};

use super::option::InfoOption;

///
/// cargo run -- info /home/solofo/Documents/books/
/// 
pub struct AudioInfo<'a> {
    pub file_path: &'a String,
    pub info_option: &'a InfoOption,
}

impl<'a> AudioInfo<'a> {
    pub fn info(&self, tx: Sender<String>) {
        match get_audio_result(&self.info_option.base_path, &self.file_path) {
            Ok(audio) => {
                save_elastic(&audio, &self.info_option.elastic);
                tx.send(format!("\
\n------------------------------------------------------------------------
{audio}\n")).unwrap_or_default();
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

fn save_elastic(audio: &AudioResult, elastic: &Option<Elastic>) {
    if let Some(el) = elastic {
        el.insert(&audio.hash, &audio);
    }
}