use super::Request;

pub struct PutRequest {
    prefix: String,
    args: Vec<String>,
}

impl PutRequest {
    pub fn new(args: &[String]) -> Self {
        PutRequest {
            prefix: String::from("*"),
            args: args.to_vec(),
        }
    }
}

impl Request for PutRequest {
    fn as_bytes(&self) -> Vec<u8> {
        let PutRequest { prefix, args } = self;
        let mut buffer = prefix.clone().into_bytes();
        buffer.append(
            &mut args
                .iter()
                .map(|arg| arg.clone().into_bytes())
                .flat_map(|this| this)
                .collect::<Vec<_>>(),
        );
        buffer
    }
}
