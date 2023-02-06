use std::thread;
use std::io::{Read, Write};
use std::time::Duration;
use std::env;

fn is_blacklisted(ip: &str, blacklist: &[&str]) -> bool {
    for blacklisted_range in blacklist {
        let parts: Vec<&str> = blacklisted_range.split('/').collect();
        let range_ip = parts[0];
        let mask = if let Ok(mask) = parts[1].parse::<u8>() {
            mask
        } else {
            continue;
        };

        let octets: Vec<&str> = ip.split('.').collect();
        let range_octets: Vec<&str> = range_ip.split('.').collect();
        if octets.len() != 4 || range_octets.len() != 4 {
            continue;
        }

        let mut matches = true;
        for i in 0..4 {
            let octet = if let Ok(octet) = octets[i].parse::<u8>() {
                octet
            } else {
                matches = false;
                break;
            };
            let range_octet = if let Ok(range_octet) = range_octets[i].parse::<u8>() {
                range_octet
            } else {
                matches = false;
                break;
            };
            if (octet & (0xff << (8 - mask))) != (range_octet & (0xff << (8 - mask))) {
                matches = false;
                break;
            }
        }
        if matches {
            return true;
        }
    }
    false
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
        &["173.245.48.0/20", "103.21.244.0/22", "103.22.200.0/22", "103.31.4.0/22", "141.101.64.0/18", "108.162.192.0/18", "190.93.240.0/20", "188.114.96.0/20", "197.234.240.0