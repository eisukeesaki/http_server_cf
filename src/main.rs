#[allow(unused_imports)]
use std::net::{TcpListener, TcpStream};
use std::io::{Write, Error};

fn main() {
    let socket = "127.0.0.1:4221";
    let listener = match TcpListener::bind(socket) {
        Ok(l) => {
            println!("Server bound to {}", socket);
            l
        },
        Err(e) => {
            eprintln!("Could not bind to {}. {}", socket, e);
            std::process::exit(1);
        }
    };

    match listener.accept() {
        Ok((mut stream, socket)) => {
            println!("Accepted connection request from {}", socket);
            let status = "HTTP/1.1 200 OK\r\n\r\n";
            match stream.write(status.as_bytes()) {
                Ok(size_written) => println!("{} bytes written to {}",
                    size_written, socket),
                Err(e) => eprintln!("Failed to write to {} {}", socket, e)
            };
        },
        Err(e) => {
            eprintln!("Failed to accept connection from {} {}", socket, e);
            std::process::exit(1);
        }
    };
}

