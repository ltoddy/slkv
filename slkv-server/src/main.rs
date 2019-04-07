pub mod config;
pub mod storage;

use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;

use self::storage::Storage;
use crate::config::{ADDRESS, FILE_PATH};

fn main() -> io::Result<()> {
    let mut storage = init();

    {
        let listener = TcpListener::bind(ADDRESS)?;
        println!("Server listening on {} ...", ADDRESS);
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let _ = handle_client(&mut storage, stream);
                }
                Err(e) => println!("Error: {}", e),
            };
        }
    }

    Ok(())
}

fn init() -> Storage {
    if Path::new(FILE_PATH).exists() {
        Storage::load_from_file(Path::new(FILE_PATH))
            .map_err(|err| eprintln!("Can't open {}\n{}", FILE_PATH, err))
            .unwrap()
    } else {
        Storage::new()
    }
}

fn handle_client(storage: &mut Storage, mut stream: TcpStream) -> io::Result<()> {
    let mut data = [0 as u8; 1024]; // using 1014 byte buffer
    let mut arguments = Vec::with_capacity(10);

    let _ = stream
        .read(&mut data)
        .map(|size| {
            let argument = String::from_utf8_lossy(&data[..size]).to_string();
            arguments.push(argument);
        })
        .map_err(|err| {
            println!("An error occurred, terminating connection with {:?}", err);
        });

    let argument = arguments
        .iter()
        .fold(String::with_capacity(128), |mut acc, x| {
            acc.push_str(x.as_str());
            acc
        });
    let result: String = format!("{}!", dispatch(storage, argument));

    stream.write(result.as_bytes()).map_err(|err| {
        eprintln!("can't send data to client: {}", err);
        err
    })?;

    Ok(())
}

fn dispatch(storage: &mut Storage, argument: String) -> String {
    let args: Vec<String> = argument[1..].split(' ').map(|s| s.into()).collect();

    if argument.starts_with('+') {
        storage.put(args)
    } else if argument.starts_with('*') {
        storage.get(args)
    } else if argument.starts_with('-') {
        storage.delete(args)
    } else if argument.starts_with('/') {
        storage.scan(args)
    } else {
        // useless
        String::new()
    }
}
