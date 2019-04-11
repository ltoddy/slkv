// 模块级别封装,对外只暴露 Commander结构体
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

// 本可以不定义这个Commander结构体,但是如果不定义,客户端就显得太面向过程了.
pub struct Commander {}

impl Commander {
    // 自身的 get, put, delete 的实现具有极大的相似性, 本可以用一个宏生成这三个函数
    // 但是 scan 函数就不是宏生成的, 不够统一.
    pub fn new() -> Self {
        Commander {}
    }

    // e.g.  get key    or    get key1 key2 ...
    // format:  get key1 key2 ...
    // 可接受任意数量的参数.
    pub fn get(&mut self, args: Vec<String>) -> Result<(), &'static str> {
        let data = GetRequest::new(args).as_bytes();

        send_request(data)?;
        Ok(())
    }

    // e.g.  put key value    or    put key1 value1 key2 value2 ...
    // format:  put key1 key2 ...
    // 可接受任意数量的参数.
    // 最好的参数数量是偶数,如果是奇数,那么最后一个参数会被省略.
    pub fn put(&mut self, args: Vec<String>) -> Result<(), &'static str> {
        let data = PutRequest::new(args).as_bytes();

        send_request(data)?;
        Ok(())
    }

    // e.g.  delete key    or    delete key1 key2 ...
    // format:  delete key1 key2 ...
    // 可接受任意数量的参数.
    pub fn delete(&mut self, args: Vec<String>) -> Result<(), &'static str> {
        let data = DeleteRequest::new(args).as_bytes();

        send_request(data)?;
        Ok(())
    }

    // e.g. scan 0 10
    // format:  scan start end
    // 数据下标从0开始， end如果大于数据的数量,那么就全部返回(容错).
    pub fn scan(&mut self, args: Vec<String>) -> Result<(), &'static str> {
        if args.len() != 2 {
            return Err("Wrong numbers of parameters");
        }

        let begin = &args[0];
        let end = &args[1];

        // 确保数据正确性,在客户端检查一下.
        let begin = begin
            .parse::<usize>()
            .map_err(|_| "Analytical parameter error, use non-negative number.")?;
        let end = end
            .parse::<usize>()
            .map_err(|_| "Analytical parameter error, use non-negative number.")?;

        if begin > end {
            return Err("The last parameter must greater this first parameter.");
        }

        let data = ScanRequest::new(args).as_bytes();

        send_request(data)?;
        Ok(())
    }
}

// Just for clippy
impl Default for Commander {
    fn default() -> Self {
        Self::new()
    }
}
