use std::io;
use super::{Runnable, get_args_parameter};

/**
* cargo run -- info /home/solofo/Videos/text.txt
*   File information: TODO
*   Movie information: TODO
*   Read office file: TODO
*/

pub struct Info {
    file_path: String,
}

impl Runnable for Info {
    fn run(&self) -> Result<(), std::io::Error> {
        println!("    WIP {}", self.file_path);
        Ok(())
    }
}

pub fn usage() -> &'static str {
    "info [file_path]        Display file informations"
}

pub fn build_cmd(args: &Vec<String>) -> Result<Info, io::Error> {
    let file_path = get_args_parameter(
        args,
        2,
        "\nread error: 'file_path' parameter required\n"
    )?;
    
    Ok(Info {
        file_path: file_path.to_string(),
    })
}
