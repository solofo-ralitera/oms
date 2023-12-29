use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{app::commands::mserv::option::MservOption, helpers::file};

#[derive(Debug, Deserialize, Serialize)]
pub struct SummaryResult {
    pub files_count: usize,
    pub files_extension: HashMap<String, usize>,
}

pub fn movies_summary(serv_option: &MservOption) -> SummaryResult {
    let mut files_extension: HashMap<String, usize> = HashMap::new();
    let _ = file::scan_count_by_extension(&serv_option.base_path, &mut files_extension);

    return SummaryResult {
        files_count: files_extension.values().sum::<usize>(),
        files_extension: files_extension,
    }
}

