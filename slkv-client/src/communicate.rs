use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

use crate::config::ADDRESS;

pub fn send_request(data: Vec<u8>) -> Result<(), &'static str> {
    let mut stream = TcpStream::connect(ADDRESS).map_err(|_| "Connection failed.")?;

    stream
        .write(data.as_slice())
        .map_err(|_| "Failed to send data.")?;

    let mut buffer = Vec::with_capacity(1024);
    let mut reader = BufReader::new(&stream);
    reader
        .read_until(b'!', &mut buffer)
        .map_err(|_| "Failed received data.")?;

    println!(
        "{}",
        String::from_utf8(buffer)
            .map_err(|_| "Can't parse received data")?
            .trim_end_matches('!')
    );
    Ok(())
}
