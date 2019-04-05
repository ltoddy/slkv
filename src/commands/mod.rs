mod delete;
mod get;
mod put;
mod scan;

use self::delete::DeleteRequest;
use self::get::GetRequest;
use self::put::PutRequest;
use self::scan::ScanRequest;
use super::communicate::send_request;

pub trait Request {
    fn as_bytes(&self) -> Vec<u8>;
}

pub struct Commander {}

impl Commander {
    pub fn new() -> Self {
        Commander {}
    }

    pub fn get(&mut self, args: Vec<String>) -> Result<(), &'static str> {
        let data = GetRequest::new(args).as_bytes();

        send_request(data)?;
        Ok(())
    }

    pub fn put(&mut self, args: Vec<String>) -> Result<(), &'static str> {
        let data = PutRequest::new(args).as_bytes();

        send_request(data)?;
        Ok(())
    }

    pub fn delete(&mut self, args: Vec<String>) -> Result<(), &'static str> {
        let data = DeleteRequest::new(args).as_bytes();

        send_request(data)?;
        Ok(())
    }

    pub fn scan(&mut self, args: Vec<String>) -> Result<(), &'static str> {
        if args.len() != 2 {
            return Err("Wrong numbers of parameters");
        }

        let begin = &args[0];
        let end = &args[1];

        let begin = begin
            .parse::<usize>()
            .map_err(|_| "Analytical parameter error, use non-negative number.")?;
        let end = end
            .parse::<usize>()
            .map_err(|_| "Analytical parameter error, use non-negative number.")?;

        let data = ScanRequest::new(begin, end).as_bytes();

        send_request(data)?;
        Ok(())
    }
}

impl Default for Commander {
    fn default() -> Self {
        Self::new()
    }
}
