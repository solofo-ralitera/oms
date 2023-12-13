mod option;
mod request;

use std::{collections::HashMap, io::{BufReader, BufRead, Write, self}, net::{TcpListener, TcpStream}, thread};
use image::EncodableLayout;
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

        if let Ok(listener) = TcpListener::bind(&mserv_option.url) {
            println!("Server started at {}", mserv_option.url);
            for stream in listener.incoming() {
                if let Ok(tcp_stream) = stream {
                    thread::spawn(move || handle_connection(tcp_stream));
                }
            }
        } else {
            return Err(io::Error::new(
                io::ErrorKind::AddrNotAvailable, 
                format!("\nUnable to start server on {}\n", mserv_option.url)
            ));
        }

        Ok(())
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    if let Ok(request_line) = buf_reader.lines().next().unwrap_or(Ok(String::new())) {
        let re = Regex::new(r"^([A-Z]{3,7}) (/.{0,}) (HTTP/.{1,})$").unwrap();
        if let Some((_, [verb, path, _])) = re.captures(&request_line).map(|c| c.extract()) {
            let (
                status, 
                headers, 
                str_content, 
                bytes_content
             ) = request::process(path, verb);
            
            let mut s_headers = String::new();
            for (h_key, h_value) in headers {
                s_headers.push_str(&format!("{}: {}\r\n", h_key, h_value));
            }

            let response = format!("HTTP/1.1 {status}\r\n{s_headers}\r\n");
            stream.write(response.as_bytes()).unwrap();
            if let Some(o_content) = str_content {
                for mut l in o_content {
                    l.push_str("\n");
                    stream.write(l.as_bytes()).unwrap();
                }
            }
            if let Some(o_bytes) = bytes_content {
                stream.write(o_bytes.as_bytes()).unwrap();
            }
            // let _ = stream.shutdown(Shutdown::Write);
            return;
        }
    }
    // Send empty response
    stream.write_all(b"").unwrap();
}

/// Help message for this command
pub fn usage() -> &'static str {
    "\
mserv        Launch media server
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
