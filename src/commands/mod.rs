use crate::commands::get::GetRequest;
use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;

pub mod delete;
pub mod get;
pub mod put;
pub mod scan;

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
        client.write_all(GetRequest::new(args).as_bytes().as_slice())?;
        Ok(())
    }

    pub fn put(&mut self, args: &[String]) -> io::Result<()> {
        // TODO
        let mut client = TcpStream::connect(ADDRESS)?;
        let mut buffer = [0; 1024];

        client.write_all(b"+")?;
        client.read_exact(&mut buffer)?;

        for arg in args {
            client.write_all(arg.as_bytes())?;
            client.read_exact(&mut buffer)?;
        }
        Ok(())
    }

    pub fn delete(&mut self, args: &[String]) -> io::Result<()> {
        // TODO
        let mut client = TcpStream::connect(ADDRESS)?;
        let mut buffer = [0; 1024];

        client.write_all(b"+")?;
        client.read_exact(&mut buffer)?;

        for arg in args {
            client.write_all(arg.as_bytes())?;
            client.read_exact(&mut buffer)?;
        }
        Ok(())
    }

    pub fn scan(&mut self, args: &[String]) -> io::Result<()> {
        // TODO
        let mut client = TcpStream::connect(ADDRESS)?;
        let mut buffer = [0; 1024];

        client.write_all(b"+")?;
        client.read_exact(&mut buffer)?;

        for arg in args {
            client.write_all(arg.as_bytes())?;
            client.read_exact(&mut buffer)?;
        }
        Ok(())
    }
}

impl Default for Commander {
    fn default() -> Self {
        Self::new()
    }
}
