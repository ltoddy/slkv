use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;

pub mod delete;
pub mod get;
pub mod put;
pub mod scan;

pub struct Commander {
    client: TcpStream,
}

impl Commander {
    pub fn from_connection(client: TcpStream) -> Self {
        Commander { client }
    }

    pub fn get(&mut self, args: &[String]) -> io::Result<()> {
        let mut buffer = [0; 1024];

        self.client.write_all(b"+")?;
        self.client.read_exact(&mut buffer)?;

        for arg in args {
            self.client.write_all(arg.as_bytes())?;
            self.client.read_exact(&mut buffer)?;
        }
        Ok(())
    }

    #[inline]
    pub fn put(&mut self, args: &[String]) -> io::Result<()> {
        let mut buffer = [0; 1024];

        self.client.write_all(b"+")?;
        self.client.read_exact(&mut buffer)?;

        for arg in args {
            self.client.write_all(arg.as_bytes())?;
            self.client.read_exact(&mut buffer)?;
        }
        Ok(())
    }

    pub fn delete(&mut self, args: &[String]) -> io::Result<()> {
        let mut buffer = [0; 1024];

        self.client.write_all(b"+")?;
        self.client.read_exact(&mut buffer)?;

        for arg in args {
            self.client.write_all(arg.as_bytes())?;
            self.client.read_exact(&mut buffer)?;
        }
        Ok(())
    }

    pub fn scan(&mut self, args: &[String]) -> io::Result<()> {
        let mut buffer = [0; 1024];

        self.client.write_all(b"+")?;
        self.client.read_exact(&mut buffer)?;

        for arg in args {
            self.client.write_all(arg.as_bytes())?;
            self.client.read_exact(&mut buffer)?;
        }
        Ok(())
    }
}

pub trait Request {
    fn as_bytes(&self) -> Vec<u8>;
}
