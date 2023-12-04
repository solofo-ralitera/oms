use std::io;
use colored::Colorize;
use regex::Regex;

///
/// https://github.com/autozimu/colorizex/blob/master/src/main.rs
/// 
pub fn colorize(line: &str, regex: &Regex, color: (u8, u8, u8)) -> Result<String, io::Error> {
    let mut line = line;
    let mut cline = String::new();
    loop {
        if let Some(mat) = regex.find(line) {
            cline += &line[..mat.start()];
            cline += format!("{}", line[mat.start()..mat.end()].on_truecolor(color.0, color.1, color.2)).as_str();
            line = &line[mat.end()..];
        } else {
            cline += line;
            break;
        }
    }

    Ok(cline)
}
