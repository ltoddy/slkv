use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let mut data = [0 as u8; 1024]; // using 1014 byte buffer
    let mut arguments = Vec::with_capacity(10);

    loop {
        match stream.read(&mut data) {
            Ok(size) => {
                if size <= 0 {
                    break; // received all data.
                }

                stream.write(&mut data[0..size])?;

                let argument = String::from_utf8_lossy(&data[..size]).to_string();
                arguments.push(argument);
            }
            Err(err) => {
                println!("An error occurred, terminating connection with {:?}", err);
                break;
            }
        }
    }

    println!("all arguments is --> {:?}", arguments);
    Ok(())
}

const ADDRESS: &str = "0.0.0.0:2333";

fn main() -> io::Result<()> {
    {
        let listener = TcpListener::bind(ADDRESS)?;
        println!("Server listening on 0.0.0.0:2333 ...");
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    handle_client(stream)?;
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
    }

    Ok(())
}
