mod option;
mod request;

use std::{collections::HashMap, io::{Write, self, Read}, net::{TcpListener, TcpStream}, thread};
use image::EncodableLayout;
use regex::Regex;
use std::str;
use self::{option::MservOption, request::ProcessParam};
use super::Runnable;

type Result<T> = std::result::Result<T, std::io::Error>;

///
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
                "p" | "provider" => mserv_option.set_provider(value)?,
                "base-path" => mserv_option.set_basepath(value)?,
                "elastic-url" => mserv_option.set_elastic(value)?,
                "url" => mserv_option.set_url(value)?,
                "transcode-output" => mserv_option.set_transcode_output(value),
                "transcode-thread" => mserv_option.set_transcode_thread(value)?,
                arg => return Err(io::Error::new(
                    io::ErrorKind::InvalidInput, 
                    format!("\nUnkown argument {}\n", arg)
                )),
            };
        }

        if mserv_option.base_path.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput, 
                format!("\nYou must provide base-path option\n")
            ));
        }

        // Serve web
        match TcpListener::bind(&mserv_option.urls[..]) {
            Ok(listener) => {
                println!("Server started at {}", mserv_option.display_urls());
                for stream in listener.incoming() {
                    if let Ok(tcp_stream) = stream {
                        let mserv_option = mserv_option.clone();
                        thread::spawn(move || handle_connection(tcp_stream, mserv_option));
                    }
                }
            },
            Err(err) => return Err(io::Error::new(
                io::ErrorKind::AddrNotAvailable, 
                format!("\nUnable to start server on {}, {}\n", mserv_option.display_urls(), err)
            ))
        }

        Ok(())
    }
}

fn handle_connection(mut stream: TcpStream, option: MservOption) {
    let mut request_headers = vec![];
    let mut request_buf: Vec<u8> = vec![];
    let mut buf = [0 as u8; 16384]; // 16k buffer
    let mut lines = String::new();
    let mut content_length: usize = 0;

    // https://stackoverflow.com/questions/67422948/rust-reading-a-stream-into-a-buffer-till-it-is-complete
    // TODO: loop request complete
    if let Ok(size) = stream.read(&mut buf) {
        request_buf.extend(&buf[0..size]);
    }

    let _ = request_buf.as_bytes().read_to_string(&mut lines);
    if lines.is_empty() {
        let _ = stream.write_all(b"");
        let _ = stream.flush();
        return;
    }

    for line in lines.lines() {
        if line.starts_with("Content-Length") {
            if let Ok(s) = line.replace("Content-Length:", "").trim().parse::<usize>() {
                content_length = s;
            }
        }
        request_headers.push(line.to_string());
    }
    let body_content = &lines[(lines.len() - content_length)..(lines.len())].to_string();

    let re = Regex::new(r"^([A-Z]{3,7}) (/.{0,}) (HTTP/.{1,})$").unwrap();
    if let Some((_, [verb, path, _])) = re.captures(&request_headers.get(0).unwrap_or(&String::new())).map(|c| c.extract()) {
        let (
            status, 
            headers, 
            str_content, 
            bytes_content
        ) = request::process(&ProcessParam {
            path: path, 
            verb: verb, 
            request_header: &request_headers,
            body_content: &body_content,
            serv_option: &option,
        });
        
        let mut s_headers = String::new();
        for (h_key, h_value) in headers {
            s_headers.push_str(&format!("{}: {}\r\n", h_key, h_value));
        }

        let response = format!("HTTP/1.1 {status}\r\n{s_headers}\r\n");
        stream.write(response.as_bytes()).unwrap();
        if let Some(content) = str_content {
            for mut l in content {
                l.push_str("\n");
                stream.write(l.as_bytes()).unwrap();
            }
        }
        if let Some(bytes) = bytes_content {
            if let Err(err) = stream.write (&bytes) {
                println!("Stream write error: {}", err.to_string());
            }
        }
        let _ = stream.flush();
        return;
    }
    // Send empty response
    let _ = stream.write_all(b"");
    let _ = stream.flush();
}

/// Help message for this command
pub fn usage() -> &'static str {
    "\
mserv --elastic-url=<string> --url=<string>  --cache-path=<string> --base-path=<string>
    Launch media server
    Prerequisites:
        - Transcode your video files into a streamable format (like H.264)
        - Elsastic search for indexing, with cors allowed, see elastic configuration
        - [Optional] ffmpeg and ffprobe, for video duration
        - [Optional] ImageMagick (convert + ghostscript) with pdf enabled, for pdf and image thumbnail
        - [Optional] exiftool to update video metadata

    --help
    --cache-path=<string>   Cache path, default ./.oms/
    --base-path=<string>   Dir path of relative root
    -p <string> --provider=<string>   possible value: local, api (default)
        If you use api, set the environment variables 
            TMDB_ACCESS_TOKEN, you can get one here https://developer.themoviedb.org/v4/reference/auth-create-access-token)
            OMDB_KEY here https://www.omdbapi.com/apikey.aspx
    --elastic-url=<string>  Url of elastic search server (with index, e.g. http://localhost:9200/oms)
    --url=<string> without http:// e.g. localhost:7777 or 192.168.33.106:7777
    --transcode-output=<string>   Extension of transcode feature (e.g. webm)
    --transcode-thread=<int>    Number of max thread used
"
}

fn print_usage() {
    println!("\n{}\n", usage());
}

pub fn build_cmd(_: &Vec<String>, options: HashMap<String, String>) -> Result<Mserv> {
    Ok(Mserv {
        cmd_options: options,
    })
}
