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

#[derive(Debug, Default)]
pub enum HTTPStatus {
    #[default]
    OK,
    BadRequest,
}

#[derive(Debug)]
pub struct Response {
    status: HTTPStatus,
    headers: HashMap::<String, String>,
    body: String,
    version: String,
}

impl Default for Response {
    fn default() -> Self {
        Self {
            status: HTTPStatus::default(),
            headers: HashMap::new(),
            body: String::new(),
            version: String::from("HTTP/1.1"),
        }
    }
}

impl ToString for Response {
    fn to_string(&self) -> String {
        let mut resp = String::new();

        let (code, message) = match self.status {
            HTTPStatus::OK => (200, "OK"),
            HTTPStatus::BadRequest => (400, "Bad Request")
        };

        resp.push_str(format!("{} {} {}\r\n", self.version, code, message).as_str());

        for (key, val) in self.headers.iter() {
            resp.push_str(format!("{}: {}\r\n", key, val).as_str());
        }

        resp.push_str("\r\n");
        resp.push_str(&self.body);

        resp
    }
}

impl Response {
    fn add_header(&mut self, key: &str, value: &str) {
        self.headers.insert(key.into(), value.into());
    }

    fn add_body(&mut self, body: String) {
        self.body = body;
        self.add_header("Content-Type", "text/html");
        self.add_header("Content-Length", &self.body.len().to_string());
    }
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

    if let Some(req) = request {
        match req.method {
            RequestMethod::GET => {
                println!("{:?} {}", req.method, req.path);

                let mut response = Response::default();
                response.add_body("<h1>Hello World!</h1>".into());

                stream.write_all(response.to_string().as_bytes())
                    .unwrap();
            },
            _ => panic!("Method not implemented.")
        }
    } else {
        stream.shutdown(Shutdown::Both).unwrap();
    }
}