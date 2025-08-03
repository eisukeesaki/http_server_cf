#[allow(unused_imports)]
use std::net::{TcpListener, TcpStream};
use std::io::{Write, Error};

// fn getSockAddr(stream: TcpStram) -> Result<, Error> {
//
// }

fn handle_connection(mut stream: TcpStream) -> Result<(), Error> {
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    let addr = match stream.peer_addr() {
        Ok(addr) => addr,
        Err(e) => {
            println!("Failed to get address from TcpStream. {}", e);
            return Err(e);
        }
    };
    match stream.write_all(response.as_bytes()) {
        Ok(_) => {
            println!("Written `{}` to {:?}", response, addr);
            Ok(())
        },
        Err(e) => {
            println!("Failed to write to {:?}.", addr);
            Err(e)
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Accepted connection request from {:?}",
                    stream.peer_addr().unwrap());
                match handle_connection(stream) {
                    Ok(_) => println!("Response sent."),
                    Err(e) =>  println!("Failed to send response. {}", e),
                };
            },
            Err(e) => println!("Failed to connect. {}", e),
        }
    }
}

