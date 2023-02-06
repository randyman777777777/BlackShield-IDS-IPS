use std::thread;
use std::io::{Read, Write};
use std::time::Duration;
use std::env;

fn is_blacklisted(ip: &str, blacklist: &[&str]) -> bool {
    blacklist.iter().any(|b| b == ip)
}

fn handle_connection(mut stream: TcpStream, blacklist: &[&str]) {
    let peer_ip = stream.peer_addr().unwrap().ip().to_string();
    if is_blacklisted(&peer_ip, blacklist) {
        println!("Blocked connection from blacklisted IP: {}", &peer_ip);
        return;
    }
    let mut buffer = [0; 512];
    let _ = stream.read(&mut buffer);
    println!("Received data from {}: {}", &peer_ip, String::from_utf8_lossy(&buffer[..]));
    let response = b"HTTP/1.1 200 OK\r\n\r\n";
    let _ = stream.write(response);
    let _ = stream.flush();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let blacklist = if args.len() > 1 {
        &args[1..]
    } else {
        &["bad_ip_1", "bad_ip_2"]
    };

    let listener = TcpListener::bind("0.0.0.0:80").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_connection(stream, blacklist)
                });
            }
            Err(e) => {
                println!("Failed to establish connection: {}", e);
            }
        }
    }
    drop(listener);
}