#[allow(unused_imports)]
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

// TODO: automatically discern status from code
fn send_status_line(mut stream: TcpStream, code: &str, status: &str) {
    let version = "HTTP/1.1";
    let status_line = format!("{} {} {}\r\n\r\n", version, code, status);
    let response = status_line.as_bytes();
    match stream.write(response) {
        Ok(size_written) => println!("{} bytes written", size_written),
        Err(e) => eprintln!("Failed to write to {}", e)
    }
}

fn extract_request_target(buff: [u8; 256]) -> String {
    let request_line = String::from_utf8_lossy(&buff);
    let lines: Vec<&str> = request_line.lines().collect();
    let request_line = lines[0];
    let request_target = request_line.split_whitespace().collect::<Vec<_>>()[1];
    return String::from(request_target);
}

fn handle_client(mut stream: TcpStream) {
    let mut buff = [0; 256];
    match stream.read(&mut buff) {
        Ok(_bytes_read) => {
            let request_target = extract_request_target(buff);
            if request_target == "/" {
                send_status_line(stream, "200", "OK");
            }
            else {
                send_status_line(stream, "404", "Not Found");
            }
        }
        Err(e) => eprintln!("{}", e)
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
        Err(e) => eprintln!("Failed to accept connection. {}", e)
    }
}

