use std::io::Result;
use std::net::TcpListener;
use std::thread;

use rserver::http::handle_request;

fn main() -> Result<()> {
    let url = "127.0.0.1:8000";
    let listener = TcpListener::bind(url)?;

    println!("Listening: {url}");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(move || {
            handle_request(stream)
        });
    }

    Ok(())
}
