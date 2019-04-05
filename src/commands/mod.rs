mod delete;
mod get;
mod put;
mod scan;

use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;

use self::delete::DeleteRequest;
use self::get::GetRequest;
use self::put::PutRequest;
use self::scan::ScanRequest;

const ADDRESS: &str = "localhost:2333";

pub trait Request {
    fn as_bytes(&self) -> Vec<u8>;
}

pub struct Commander {}

impl Commander {
    pub fn new() -> Self {
        Commander {}
    }

    pub fn get(&mut self, args: &[String]) -> io::Result<()> {
        let mut client = TcpStream::connect(ADDRESS)?;
        let data = GetRequest::new(args).as_bytes();
        client.write_all(data.as_slice())?;
        Ok(())
    }

    pub fn put(&mut self, args: &[String]) -> io::Result<()> {
        let mut client = TcpStream::connect(ADDRESS)?;
        let data = PutRequest::new(args).as_bytes();
        client.write_all(data.as_slice())?;
        Ok(())
    }

    pub fn delete(&mut self, args: &[String]) -> io::Result<()> {
        let mut client = TcpStream::connect(ADDRESS)?;
        let data = DeleteRequest::new(args).as_bytes();
        client.write_all(data.as_slice())?;
        Ok(())
    }

    pub fn scan(&mut self, args: &[String]) -> io::Result<()> {
        let mut client = TcpStream::connect(ADDRESS)?;

        if args.len() != 2 {}
        let [begin, end] = args[..2];
        let begin = begin
            .parse::<usize>()
            .map_err(|_| Err("Analytical parameter error, use non-negative number."))?;
        let end = end
            .parse::<usize>()
            .map_err(|_| Err("Analytical parameter error, use non-negative number."))?;

            let data = ScanRequest::new(begin, end).as_bytes();
        client.write_all(data.as_slice())?;
        Ok(())
    }
}

impl Default for Commander {
    fn default() -> Self {
        Self::new()
    }
}
