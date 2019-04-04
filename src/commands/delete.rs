use super::Request;

pub struct DeleteRequest {
    prefix: String,
    args: Vec<String>,
}

impl DeleteRequest {
    pub fn new(args: &[String]) -> Self {
        DeleteRequest {
            prefix: String::from("*"),
            args: args.to_vec(),
        }
    }
}

impl Request for DeleteRequest {
    fn as_bytes(&self) -> Vec<u8> {
        let DeleteRequest { prefix, args } = self;
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
