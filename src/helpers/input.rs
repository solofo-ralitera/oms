use std::collections::HashMap;
use regex::Regex;
use super::cache;


///
/// TODO: parse options: https://stackoverflow.com/questions/15619320/how-can-i-access-command-line-parameters-in-rust
/// 
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

