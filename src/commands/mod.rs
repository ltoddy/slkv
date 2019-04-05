mod delete;
mod get;
mod put;
mod scan;

use std::io::Write;
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

    pub fn get(&mut self, args: &[String]) -> Result<(), &'static str> {
        let data = GetRequest::new(args).as_bytes();

        let mut client = TcpStream::connect(ADDRESS).map_err(|_| "Connection failed.")?;
        client
            .write_all(data.as_slice())
            .map_err(|_| "Failed to send data.")?;
        Ok(())
    }

    pub fn put(&mut self, args: &[String]) -> Result<(), &'static str> {
        let data = PutRequest::new(args).as_bytes();

        let mut client = TcpStream::connect(ADDRESS).map_err(|_| "Connection failed.")?;
        client
            .write_all(data.as_slice())
            .map_err(|_| "Failed to send data.")?;

        Ok(())
    }

    pub fn delete(&mut self, args: &[String]) -> Result<(), &'static str> {
        let data = DeleteRequest::new(args).as_bytes();

        let mut client = TcpStream::connect(ADDRESS).map_err(|_| "Connection failed.")?;
        client
            .write_all(data.as_slice())
            .map_err(|_| "Failed to send data.")?;

        Ok(())
    }

    pub fn scan(&mut self, args: &[String]) -> Result<(), &'static str> {
        if args.len() != 2 {
            return Err("Wrong numbers of parameters");
        }

        let begin = args.get(0).unwrap();
        let end = args.get(1).unwrap();

        let begin = begin
            .parse::<usize>()
            .map_err(|_| "Analytical parameter error, use non-negative number.")?;
        let end = end
            .parse::<usize>()
            .map_err(|_| "Analytical parameter error, use non-negative number.")?;

        let data = ScanRequest::new(begin, end).as_bytes();

        let mut client = TcpStream::connect(ADDRESS).map_err(|_| "Connection failed.")?;
        client
            .write_all(data.as_slice())
            .map_err(|_| "Failed to send data.")?;

        Ok(())
    }
}

impl Default for Commander {
    fn default() -> Self {
        Self::new()
    }
}
