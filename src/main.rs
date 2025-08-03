#[allow(unused_imports)]
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4242").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(_) => println!("connection accepted"),
            Err(e) => println!("failed to connect. {}", e),
        }
    }
}

