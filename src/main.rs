use std::io::{Result, Write, BufRead, BufReader};
use std::net::TcpListener;

fn main() -> Result<()> {
    let url = "127.0.0.1:8000";
    let listener = TcpListener::bind(url)?;

    println!("Listening: {url}");

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        let buf_reader = BufReader::new(&mut stream);
        let _request_line = buf_reader.lines().next().unwrap().unwrap();
        let body = "Hello, world!";

        let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}\r\n", body.len(), body);
        stream.write_all(response.as_bytes()).unwrap();
    }

    Ok(())
}
