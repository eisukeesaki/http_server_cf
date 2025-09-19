// #[allow(unused_imports)]
// use std::net::{TcpListener, TcpStream};
// use std::io::{Read, Write};
//
// // TODO: automatically discern status_text from status_code
// fn send_status_line(mut stream: TcpStream, status_code: &str, status_text: &str) {
//     let version = "HTTP/1.1";
//     let status_line = format!("{} {} {}\r\n\r\n", version, status_code, status_text);
//     let response = status_line.as_bytes();
//     match stream.write(response) {
//         Ok(size_written) => println!("{} bytes written", size_written),
//         Err(e) => eprintln!("Failed to write to {}", e)
//     }
// }
//
// // fn extract_request_target(buff: [u8; 256]) -> Vec<&str> {
// fn extract_request_target(buff: [u8; 256]) {
//     let request_line = String::from_utf8_lossy(&buff);
//     let lines: Vec<&str> = request_line.lines().collect();
//     let request_line = lines[0];
//     println!("{:?}", request_line);
//     // create an empty Vec<&str>
//     // let request_target = Vec<String> = Vec::new();
//     // find first occurence of / in request_line
//     // read from the / to the next occurence of ' '
//     //     if another / is found before the ' ',
//     //         push the chars starting from / (inclusive) to the next / (exclusive)
//     //     otherwise push the chars starting from / (inclusive) to the final character before the ' '
// }
//
// // fn handle_endpoints(request_target: Vec<&str>) {
// //     // the Vec<&str> looks like:
// //     // ["/"]
// //     // ["/echo", "/abc"]
// //
// //     if ["/"] {
// //         send_status_line(stream, "200", "OK");
// //     }
// //     else if request_target[0] == "/echo" {
// //         include &request_target[1..] as response body
// //         send_status_line(stream, "200", "OK");
// //     }
// //     else {
// //         send_status_line(stream, "404", "Not Found");
// //     }
// // }
//
// fn handle_client(mut stream: TcpStream) {
//     let mut buff: [u8; 256] = [0; 256];
//     match stream.read(&mut buff) {
//         Ok(_bytes_read) => {
//             let _request_target = extract_request_target(buff);
//             // handle_endpoints(request_target);
//         }
//         Err(e) => eprintln!("{}", e)
//     }
// }
//
// fn main() {
//     let socket = "127.0.0.1:4221";
//     let listener = match TcpListener::bind(socket) {
//         Ok(listener) => listener,
//         Err(e) => {
//             eprintln!("Could not bind to socket. {}", e);
//             std::process::exit(1);
//         }
//     };
//
//     match listener.accept() {
//         Ok((stream, _)) => handle_client(stream),
//         Err(e) => eprintln!("Failed to accept connection. {}", e)
//     }
// }
//
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::string::String;

fn handle_connection(mut stream: TcpStream) {
    let mut buff = Vec::new();
    buff.resize(512, 0);
    let bytes_read = match stream.read(&mut buff) {
        Ok(bytes_read) => bytes_read,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };
    let request = String::from_utf8_lossy(&buff[..bytes_read]);
    let request_line = match request.lines().next() {
        Some(line) => line,
        None => return
    };
    println!("{}", request_line);
    let req_tgt: String = request_line
        .split(' ')
        .collect::<Vec<_>>()[1]
        .to_string(); // contains something like: /echo/alohomora
    // check presence of / at req_tgt[0]
    // if not found, return
    let end = match req_tgt[1..].find('/') {
        Some(end) => 1 + end,
        None => req_tgt.len()
    };
    let lvl1 = &req_tgt[1..end];
    println!("{}", lvl1);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 || 3 < args.len() {
        println!("usage: ./codecrafters-http-server \"ipaddr\" \"port\"");
        return;
    }
    let ipaddr = &args[1];
    let port = &args[2];
    let socket = format!("{}:{}", ipaddr, port);
    let listener = match TcpListener::bind(socket) {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("Could not bind to socket. {}", e);
            std::process::exit(1);
        }
    };

    match listener.accept() {
        Ok((stream, _)) => handle_connection(stream),
        Err(e) => eprintln!("Failed to accept connection. {}", e)
    }
}

