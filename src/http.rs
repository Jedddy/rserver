use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpStream, Shutdown};

#[derive(Debug)]
pub enum RequestMethod {
    GET,
    POST,
    PUT,
    DELETE
}

impl From<&str> for RequestMethod {
    fn from(string: &str) -> Self {
        match string {
            "GET" => Self::GET,
            "POST" => Self::POST,
            "PUT" => Self::PUT,
            "DELETE" => Self::DELETE,
            _ => panic!("Invalid method received.")
        }
    }
}

#[derive(Debug)]
pub struct Request {
    pub method: RequestMethod,
    pub path: String,
    pub version: String,
    pub headers: HashMap<String, String>,
}

pub fn parse_request(stream: &TcpStream) -> Option<Request> {
    let buffer = BufReader::new(stream);
    let mut lines = buffer.lines();
    if let Some(info) = lines.next() {
        let info = info.unwrap();
        let first = info
            .split_whitespace()
            .take(3)
            .collect::<Vec<_>>();

        let method = RequestMethod::from(first[0]);
        let path = first[1].to_string();
        let version = first[2].to_string();
        let mut headers = HashMap::new();

        while let Some(line) = lines.next() {
            let line = line.unwrap();

            if line.trim().is_empty() || line == "\n\n" {
                break;
            }

            let parts = line.split(":").collect::<Vec<&str>>();

            let key = parts[0].trim().into();
            let value = parts[1].trim().into();

            headers.insert(key, value);
        }

        return Some(Request { method, path, headers, version });
    }

    None
}

pub fn handle_request(mut stream: TcpStream) {
    let request = parse_request(&stream);
    println!("{:?}", request);
    if let Some(req) = request {
        match req.method {
            RequestMethod::GET => {
                stream.write_all(b"HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n<h1>Hello World!</h1>")
                    .unwrap();
            },
            _ => panic!("Method not implemented.")
        }
    } else {
        stream.shutdown(Shutdown::Both).unwrap();
    }
}