pub mod string;
pub mod file;
pub mod pdf;
pub mod output;

use std::{thread, time::Duration};

/// Pause the thread for x millis
/// 
/// # Arguments
///
/// * `millis` - pause duration in millis
/// 
/// # Examples
/// 
/// ```
/// use oms::helpers;
/// helpers::sleep(0);
/// ```
pub fn sleep(millis: u64) {
    thread::sleep(Duration::from_millis(millis));
}
