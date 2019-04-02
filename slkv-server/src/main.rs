use std::io::{self, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 1024]; // using 1014 byte buffer
    while match stream.read(&mut data) {
        Ok(size) if size > 0 => {
            stream
                .write(&data)
                .map_err(|err| {
                    println!("------------> {:?}", err);
                    err
                })
                .unwrap();
            println!(" ===> received size: {}", size);
            let arguments = unsafe { String::from_utf8_unchecked(data[..size].to_vec()) };
            println!("==> received: {:?}", arguments);
            true
        }
        Err(_) => {
            println!(
                "An error occurred, terminating connection with {}",
                stream.peer_addr().unwrap()
            );
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
        _ => false,
    } {}
}

const ADDRESS: &str = "0.0.0.0:2333";

fn main() -> io::Result<()> {
    {
        let listener = TcpListener::bind(ADDRESS)?;
        println!("Server listening on 0.0.0.0:2333 ...");
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("New connection: {}", stream.peer_addr()?);
                    handle_client(stream);
                    println!("disconnect ...");
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
    }

    Ok(())
}
