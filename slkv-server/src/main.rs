pub mod persistence;
pub mod storage;

use std::fs::File;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path;

use std::path::Path;
use storage::Storage;

const ADDRESS: &str = "0.0.0.0:2333";
const FILE_PATH: &str = "data.sldb";

fn main() -> io::Result<()> {
    let mut storage = init();

    {
        let listener = TcpListener::bind(ADDRESS)?;
        println!("Server listening on {} ...", ADDRESS);
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => handle_client(&mut storage, stream)?,
                Err(e) => println!("Error: {}", e),
            }
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

    loop {
        match stream.read(&mut data) {
            Ok(size) => {
                if size == 0 {
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

    let argument = arguments
        .iter()
        .fold(String::with_capacity(128), |mut acc, x| {
            acc.push_str(x.as_str());
            acc
        });
    dispatch(storage, argument);
    Ok(())
}

fn dispatch(storage: &mut Storage, argument: String) -> String {
    let args: Vec<String> = argument[1..].split(" ").map(|s| s.into()).collect();

    if argument.starts_with("+") {
        return storage.put(args);
    } else if argument.starts_with("*") {
        return storage.get(args);
    } else if argument.starts_with("-") {
        return storage.delete(args);
    } else if argument.starts_with("/") {
        // TODO
        println!("SCAN    ==> {}", argument);
        return String::from("Scan");
    } else {
        return String::from("Ok");
    }
}
