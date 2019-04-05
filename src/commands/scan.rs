use super::Request;

pub struct ScanRequest {
    prefix: String,
    begin: usize,
    end: usize,
}

impl ScanRequest {
    pub fn new(begin: usize, end: usize) -> Self {
        ScanRequest {
            prefix: String::from("/"),
            begin,
            end,
        }
    }
}

impl Request for ScanRequest {
    fn as_bytes(&self) -> Vec<u8> {
        let Self { prefix, begin, end } = self;
        let mut buffer = prefix.clone().into_bytes();
        buffer.append(&mut begin.to_be_bytes().to_vec());
        buffer.append(&mut end.to_be_bytes().to_vec());
        buffer
    }
}
