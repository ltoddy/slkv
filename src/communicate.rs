use std::io::Write;
use std::net::TcpStream;

const ADDRESS: &str = "localhost:2333";

pub fn send_request(data: Vec<u8>) -> Result<(), &'static str> {
    let mut client = TcpStream::connect(ADDRESS).map_err(|_| "Connection failed.")?;
    client
        .write_all(data.as_slice())
        .map_err(|_| "Failed to send data.")?;
    Ok(())
}
