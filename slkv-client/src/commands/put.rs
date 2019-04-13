use super::Request;

pub struct PutRequest {
    prefix: String,
    args: Vec<String>,
}

impl PutRequest {
    pub fn new(args: Vec<String>) -> Self {
        PutRequest {
            prefix: String::from("+"),
            args,
        }
    }
}

impl Request for PutRequest {
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
    use super::PutRequest;

    #[test]
    pub fn test_put_request() {
        let args = vec![];
        let request = PutRequest::new(args);

        assert_eq!(request.as_bytes(), b"+");

        let args = vec!["k1".to_string(), "v1".to_string()];
        let request = PutRequest::new(args);

        assert_eq!(request.as_bytes(), b"+k1 v1");

        let args = vec![
            "k1".to_string(),
            "v1".to_string(),
            "k2".to_string(),
            "v2".to_string(),
        ];
        let request = PutRequest::new(args);

        assert_eq!(request.as_bytes(), b"+k1 v1 k2 v2");

        let args = vec![
            "k1".to_string(),
            "v1".to_string(),
            "k2".to_string(),
            "v2".to_string(),
        ];
        let request = PutRequest::new(args);

        assert_eq!(request.as_bytes(), b"+k1 v1 k2 v2");
    }
}
