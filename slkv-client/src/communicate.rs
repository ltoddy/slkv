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

    // 这里, 在这个函数中直接打印出来了, 这并不好, 最好的方式是把接收到的response返回出去,让调用方去print.
    // 不过我并没有怎么见过一个函数返回 Result<String, &'static str>.
    println!(
        "{}",
        String::from_utf8(buffer)
            .map_err(|_| "Can't parse received data")?
            .trim_end_matches('!')
    );
    Ok(())
}
