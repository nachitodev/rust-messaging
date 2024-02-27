use std::io::Read;
use std::net::{TcpListener, TcpStream};

const MESSAGE_SIZE: usize = 128;

pub fn handle_client(mut stream: TcpStream) {
    let rx_bytes: &mut [u8] = &mut [0; MESSAGE_SIZE];
    let _ = stream.read(rx_bytes);
    let received = std::str::from_utf8(&rx_bytes);

    println!("Client addr: {:?}", stream.peer_addr().unwrap());
    println!(
        "Received Client data: {:?}",
        received.unwrap().trim_end().to_string()
    );
}

pub fn main() -> std::io::Result<()> {
    let listener: TcpListener = TcpListener::bind("0.0.0.0:8080")?;
    println!(
        "TCP Server listening on {:?}",
        listener.local_addr().unwrap()
    );

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                println!("There was an error connecting, {}", e);
            }
        }
    }
    Ok(())
}
