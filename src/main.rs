extern crate colored;

use std::net::TcpStream;
use std::net::Shutdown;
use colored::*;

fn main() {

    let mut index = 1;
    println!("{}", "Online device {".bold());

    while index < 255 {
    
        let address = format!("192.168.1.{}:22", index.to_string());
    
        if let Ok(conn) = TcpStream::connect(&address) {
            println!("\t{}", address);
            conn.shutdown(Shutdown::Both)
                .expect("Shutdown call failed");
        }
        index = index + 1;
    }
    println!("{}", "}".bold());
}
