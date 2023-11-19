pub mod string;
pub mod file;

use std::{thread, time::Duration};

/// Pause the thread for x millis
/// 
/// # Arguments
///
/// * `millis` - pause duration in millis
pub fn sleep(millis: u64) {
    thread::sleep(Duration::from_millis(millis));
}
