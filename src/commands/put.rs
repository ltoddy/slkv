use super::Request;

pub struct PutRequest {
    prefix: String,
    args: Vec<String>,
}

impl PutRequest {
    pub fn new(args: &[String]) -> Self {
        PutRequest {
            prefix: String::from("+"),
            args: args.to_vec(),
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
