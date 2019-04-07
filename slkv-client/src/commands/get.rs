use super::Request;

pub struct GetRequest {
    prefix: String,
    args: Vec<String>,
}

impl GetRequest {
    pub fn new(args: Vec<String>) -> Self {
        GetRequest {
            prefix: String::from("*"),
            args,
        }
    }
}

impl Request for GetRequest {
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