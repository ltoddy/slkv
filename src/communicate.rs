//use std::io;
//use std::net::TcpStream;
//
//pub struct Communicator {
//    addr: &'static str,
//    client: TcpStream,
//}
//
//impl Communicator {
//    pub fn connect(addr: &'static str) -> Self {
//        let client = TcpStream::connect(addr)
//            .map_err(|err| {
//                println!("can't connect slkv server.\n  {:?}", err);
//                err
//            })
//            .unwrap();
//
//        Communicator { addr, client }
//    }
//}
