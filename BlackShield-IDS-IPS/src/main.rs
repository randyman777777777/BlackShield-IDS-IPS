use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn handle_connection(mut stream: TcpStream, blacklist: &[&str]) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);

    for line in request.lines() {
        for entry in blacklist {
            if line.contains(entry) {
                let response = "HTTP/1.1 403 Forbidden\r\n\r\n";
                stream.write(response.as_bytes()).unwrap();
                stream.flush().unwrap();
                return;
            }
        }
    }
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let blacklist = [
        "173.245.48.0/20", 
        "103.21.244.0/22", 
        "103.22.200.0/22", 
        "103.31.4.0/22", 
        "141.101.64.0/18", 
        "108.162.192.0/18", 
        "190.93.240.0/20", 
        "188.114.96.0/20", 
        "197.234.240.0/22",
    ];
    
    let listener = TcpListener::bind("0.0.0.0:80").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream, &blacklist);
    }
}