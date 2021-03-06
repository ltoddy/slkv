pub mod config;
pub mod storage;

use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;

use config::{ADDRESS, FILE_PATH};
use storage::Storage;

fn main() -> io::Result<()> {
    let mut storage = init();
    // 对于服务端来说,如果出现任何重大的异常,就让服务端挂掉.
    {
        let listener = TcpListener::bind(ADDRESS)?;
        println!("Server listening on {} ...", ADDRESS);
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let _ = handle_client(&mut storage, stream);
                }
                Err(e) => println!("Error: {}", e),
            };
        }
        // auto drop, disconnect.
    }

    Ok(())
}

fn init() -> Storage {
    if Path::new(FILE_PATH).exists() {
        Storage::load_from_file(Path::new(FILE_PATH))
            .unwrap_or_else(|_| panic!("Can't open {}", FILE_PATH))
    } else {
        Storage::new()
    }
}

fn handle_client(storage: &mut Storage, mut stream: TcpStream) -> io::Result<()> {
    let mut data = [0 as u8; 1024]; // using 1014 byte buffer
    let mut arguments = Vec::with_capacity(16);

    // 从客户端接受数据
    let _ = stream
        .read(&mut data)
        .map(|size| {
            let argument = String::from_utf8_lossy(&data[..size]).to_string();
            arguments.push(argument);
        })
        .map_err(|err| {
            println!("An error occurred, terminating connection with {:?}", err);
        });

    let argument = arguments
        .iter()
        .fold(String::with_capacity(128), |mut acc, x| {
            acc.push_str(x.as_str());
            acc
        });
    let result: String = format!("{}!", dispatch(storage, argument));

    // 讲客户端发送数据处理之后的结果返回给客户端
    stream.write(result.as_bytes()).map_err(|err| {
        eprintln!("can't send data to client: {}", err);
        err
    })?;

    Ok(())
}

// 调度客户端发送过来的数据
fn dispatch(storage: &mut Storage, argument: String) -> String {
    let args: Vec<String> = argument[1..].split(' ').map(|s| s.into()).collect();

    if argument.starts_with('+') {
        storage.put(args)
    } else if argument.starts_with('*') {
        storage.get(args)
    } else if argument.starts_with('-') {
        storage.delete(args)
    } else if argument.starts_with('/') {
        storage.scan(args)
    } else {
        // ignore wrong command.
        // 因为客户端已经做过检查,所以这里是走不到的.
        String::new()
    }
}

#[cfg(test)]
pub mod test {
    use super::dispatch;
    use super::Storage;

    #[test]
    pub fn test_dispatch_arguments() {
        let mut storage = Storage::new();
        // put
        let argument = String::from("+key1 value1");
        let res = dispatch(&mut storage, argument);
        assert_eq!(res, "Ok\n");

        // get
        let argument = String::from("*key1 non-exist");
        let res = dispatch(&mut storage, argument);
        assert_eq!(res, "key1 => value1\nnon-exist => None\n");

        // delete
        let argument = String::from("-key1");
        let res = dispatch(&mut storage, argument);
        assert_eq!(res, "Ok\n");

        // scan
        let argument = String::from("/0 10");
        let res = dispatch(&mut storage, argument);
        assert_eq!(res, "");
    }
}
