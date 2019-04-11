use super::Request;

pub struct ScanRequest {
    prefix: String,
    args: Vec<String>,
}

impl ScanRequest {
    pub fn new(args: Vec<String>) -> Self {
        ScanRequest {
            prefix: String::from("/"),
            args,
        }
    }
}

impl Request for ScanRequest {
    fn as_bytes(&self) -> Vec<u8> {
        let Self { prefix, args } = self;
        let mut buffer = prefix.clone().into_bytes();
        buffer.append(
            &mut args
                .iter()
                .enumerate()
                .map(|(index, arg)| {
                    let mut temp = arg.clone().into_bytes();
                    if index + 1 != args.len() {
                        temp.push(32);
                    }
                    temp
                })
                .flat_map(|this| this)
                .collect::<Vec<_>>(),
        );
        buffer
    }
}

#[cfg(test)]
pub mod test {
    use super::super::Request;
    use super::ScanRequest;

    #[test]
    pub fn test_scan_request() {
        let args = vec!["0".to_string(), "1".to_string()];
        let request = ScanRequest::new(args);

        assert_eq!(request.as_bytes(), b"/0 1");
    }
}
