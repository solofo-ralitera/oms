use std::{collections::HashMap, io::{self, Write}, cmp::max};
use regex::Regex;
use super::cache;

pub fn read_line(message: &str) -> String {
    if !message.is_empty() {
       print!("{message}");
       io::stdout().flush().unwrap_or_default();
    }
    let mut line_value = String::new();
    match io::stdin().read_line(&mut line_value) {
       Ok(_) => line_value.trim().to_string(),
       Err(_) => "".to_string(),
    }
 }
 
///
/// TODO: parse options: https://stackoverflow.com/questions/15619320/how-can-i-access-command-line-parameters-in-rust
pub fn parse_command_option(args: &Vec<String>) -> HashMap<String, String> {
    let mut options = HashMap::new();
    for (index, option) in args.iter().enumerate() {
        if option.starts_with("-") {
            // Example -tupln
            if Regex::new("^-[a-zA-Z]{2,}").unwrap().is_match(option) {
                for c in option.chars().skip(1) {
                    options.insert(c.to_string(), "".to_string());
                }
            }
            // Example --help, -h ...
            else {
                let re: Regex = Regex::new("^-{1,2}").unwrap();
                let mut option = re.replace(option, "").to_string();
                
                let mut next_value: String;

                // case --extension=txt
                if option.contains("=") {
                    let co: String = option.clone();
                    let mut concated_value: Vec<String> = vec![];
                    for (idx, opt) in co.split("=").enumerate() {
                        if idx == 0 {
                            option = opt.to_string();
                        } else {
                            concated_value.push(opt.to_string());
                        }
                    }
                    next_value = concated_value.join("=");
                } else {
                    // Get next value as parameter of option
                    next_value = args.get(index + 1).unwrap_or(&String::new()).clone();
                    // If next value is an option, ignore this value
                    if next_value.starts_with("-") {
                        next_value = "".to_string();
                    }
                }
                // Common option, ignored in final result
                if option.eq("cache-path") {
                    cache::set_base_path(&next_value);
                    continue;
                }
                options.insert(option.clone(), next_value);
            }
        }
    }
    return options;
}


// https://docs.rs/warp-range/latest/src/warp_range/lib.rs.html#1-148
pub fn get_range_params(request_header: &Vec<String>, size: u64) -> Result<(u64, u64), io::Error> {
    let range = request_header.iter().filter(|line| line.starts_with("Range:")).next();
    match range {
        Some(range) => {
            let range: Vec<String> = range
                .replace("Range:", "")
                .replace("bytes=", "")
                .trim()
                .split("-")
                .filter_map(|n| if n.len() > 0 {Some(n.to_string())} else {None})
                .collect();
            let start = if range.len() > 0 { 
                range[0].parse::<u64>().unwrap_or_default()
            } else { 
                0 
            };
            let end = if range.len() > 1 {
                range[1].parse::<u64>().unwrap_or_default()
            } else {
                if size <= 0 {
                    0
                } else {
                    max(0, size - 1)
                }
            };
            Ok((start, end))
        },
        None => Ok((0, size - 1))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_range_params_0() {
        let headers = vec![
            "Range: bytes=0-".to_string(),
        ];
        match get_range_params(&headers, 1_000_000) {
            Ok(range) => {
                assert_eq!((0, 999_999), range);
            },
            Err(_) => {
                panic!("get_range_params should return tulpe");

            }
        }
    }

    #[test]
    fn get_range_params_0_100() {
        let headers = vec![
            "Range: bytes=0-100".to_string(),
        ];
        match get_range_params(&headers, 1_000_000) {
            Ok(range) => {
                assert_eq!((0, 100), range);
            },
            Err(_) => {
                panic!("get_range_params should return tulpe");

            }
        }
    }

    #[test]
    fn get_range_params_nil() {
        let headers = vec![];
        match get_range_params(&headers, 1_000_000) {
            Ok(range) => {
                assert_eq!((0, 999_999), range);
            },
            Err(_) => {
                panic!("get_range_params should return tulpe");

            }
        }
    }
}