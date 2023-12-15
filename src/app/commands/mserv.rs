mod option;
mod request;

use std::{collections::HashMap, io::{BufReader, BufRead, Write, self}, net::{TcpListener, TcpStream}, thread};
use regex::Regex;

use self::option::MservOption;

use super::Runnable;

type Result<T> = std::result::Result<T, std::io::Error>;

///
/// 
/// cargo run -- mserv --elastic-dsn="http://localhost:9200" --url="localhost:7878"  --cache-path="/media/solofo/MEDIA/.oms"
/// 
pub struct Mserv {
    /// Command options
    cmd_options: HashMap<String, String>,
}

impl Runnable for Mserv {
    fn run(&self) -> Result<()> {
        let mut mserv_option = MservOption::new();

        // --help
        if self.cmd_options.contains_key("h") || self.cmd_options.contains_key("help") {
            print_usage();
            return Ok(());
        }
        for (option, value) in &self.cmd_options {
            match option.as_str() {
                "elastic-dsn" => mserv_option.set_elastic(value),
                "url" => mserv_option.set_url(value)?,
                arg => return Err(io::Error::new(
                    io::ErrorKind::InvalidInput, 
                    format!("\nUnkown argument {}\n", arg)
                )),
            };
        }

        // Serve web
        match TcpListener::bind(&mserv_option.url.to_string()) {
            Ok(listener) => {
                println!("Server started at {}", mserv_option.url);
                for stream in listener.incoming() {
                    if let Ok(tcp_stream) = stream {
                        let mserv_option = mserv_option.clone();
                        thread::spawn(move || handle_connection(tcp_stream, mserv_option));
                    }
                }
            },
            Err(err) => return Err(io::Error::new(
                io::ErrorKind::AddrNotAvailable, 
                format!("\nUnable to start server on {}, {}\n", mserv_option.url, err)
            ))
        }

        Ok(())
    }
}

fn handle_connection(mut stream: TcpStream, option: MservOption) {
    let reader = BufReader::new(&mut stream);
    let mut request_headers = vec![];
    for line in reader.lines() {
        let line = line.unwrap_or_default();
        if line.len() < 3 {
            //detect empty line
            break;
        }
        request_headers.push(line);
    }

    let re = Regex::new(r"^([A-Z]{3,7}) (/.{0,}) (HTTP/.{1,})$").unwrap();
    if let Some((_, [verb, path, _])) = re.captures(&request_headers.get(0).unwrap_or(&String::new())).map(|c| c.extract()) {
        let (
            status, 
            headers, 
            str_content, 
            bytes_content
        ) = request::process(path, verb, &request_headers);
        
        let mut s_headers = String::new();
        for (h_key, h_value) in headers {
            s_headers.push_str(&format!("{}: {}\r\n", h_key, h_value));
        }

        let response = format!("HTTP/1.1 {status}\r\n{s_headers}\r\n");
        stream.write(response.as_bytes()).unwrap();
        if let Some(o_content) = str_content {
            for mut l in o_content {
                l.push_str("\n");
                // Replace elastic dsn
                if let Some(elastic) = option.elastic.as_ref() {
                    if path.ends_with("elastic.js") {
                        l = l.replace("\"ELASTIC_URL\"", &format!("\"{}/{}\"", elastic.url, elastic.index));
                    }
                }
                stream.write(l.as_bytes()).unwrap();
            }
        }
        if let Some(o_bytes) = bytes_content {
            if let Err(err) = stream.write (&o_bytes) {
                println!("Stream write error: {}", err.to_string());
            }
        }
        stream.flush().unwrap();
        return;
    }
    // Send empty response
    stream.write_all(b"").unwrap();
    stream.flush().unwrap();
}

/// Help message for this command
pub fn usage() -> &'static str {
    "\
mserv        Launch media server
You must have ffmpeg installed if you have an .avi file that needs to be re-encoded for streaming
    --help
    --cache-path=<string>   Cache path, default ./.oms/
    --elastic-dsn=<string>  Elastic search server
    --url=<string>  
"
}

fn print_usage() {
    println!("\n{}\n", usage());
}

pub fn build_cmd(options: HashMap<String, String>) -> Result<Mserv> {
    Ok(Mserv {
        cmd_options: options,
    })
}