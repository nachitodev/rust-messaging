use std::io::prelude::*;
use std::net::TcpStream;

pub fn send_data(mut stream: TcpStream) {
    let mut input = String::new();
    let _x = 128 - std::io::stdin().read_line(&mut input).unwrap();

    let input = input + &" ".repeat(_x);
    let _ = stream.write_all(input.as_bytes());
}

pub fn connect(url: &str) {
    let stream: TcpStream = TcpStream::connect(url).unwrap();
    send_data(stream);
}

pub fn main() {
    let url: &str = "127.0.0.1:8080";
    loop {
        connect(url);
    }
}
