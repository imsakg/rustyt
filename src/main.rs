// #![allow(unused)]

use crate::prelude::*;
use clap::Parser;
use threadpool::ThreadPool;

mod error;
mod prelude;
mod threadpool;
mod utils;

use std::fs;
use std::io::prelude::*;
use std::io::{self, BufRead, Read, Write};
use std::net::TcpListener;
use std::net::TcpStream;
use std::process::Command;
use std::thread;
use std::time::Duration;

const WORKERS: usize = 4;

fn main() -> Result<()> {
    let args = Cli::parse();

    let listener = TcpListener::bind(format!("{}:{}", &args.host, &args.port)).unwrap();
    println!("Listening on {}:{}", &args.host, &args.port);

    let pool = ThreadPool::new(WORKERS);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
    println!("Shutting down.");

    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "src/template/index.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "src/template/index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "src/template/404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn download_video_extract_audio(id: String) -> Result<()> {
    let url = format!("https://www.youtube.com/watch?v={}", id);

    /* ytdl <---> ffmpeg <---> http handler */
    // exec command ytdl
    // exec command ffmpeg

    let ytdl = Command::new("youtube-dl")
        .arg("--output")
        .arg(format!("{}.%(ext)s", id))
        .arg(url)
        .arg("-o-")
        .output()
        .expect("failed to execute process");

    // !TODO

    Ok(())
}
