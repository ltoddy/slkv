use super::Request;

pub struct GetRequest {
    prefix: String,
    args: Vec<String>,
}

impl GetRequest {
    pub fn new(args: &[String]) -> Self {
        GetRequest {
            prefix: String::from("*"),
            args: args.to_vec(),
        }
    }
}

impl Request for GetRequest {
    fn as_bytes(&self) -> Vec<u8> {
        let GetRequest { prefix, args } = self;
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
