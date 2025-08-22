#[allow(unused_imports)]
use std::net::{TcpListener, TcpStream};
use std::io::Write;

fn handle_client(mut stream: TcpStream) {
    let status = "HTTP/1.1 200 OK\r\n\r\n";
    let response = status.as_bytes();
    match stream.write(response) {
        Ok(size_written) => println!("{} bytes written", size_written),
        Err(e) => eprintln!("Failed to write to {}", e)
    }
}

fn main() {
    let socket = "127.0.0.1:4221";
    let listener = match TcpListener::bind(socket) {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("Could not bind to socket. {}", e);
            std::process::exit(1);
        }
    };

    match listener.accept() {
        Ok((stream, _)) => handle_client(stream),
        Err(e) => {
            eprintln!("Failed to accept connection. {}", e);
            std::process::exit(1);
        }
    }
}

