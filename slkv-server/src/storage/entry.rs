use std::fmt::{Display, Error, Formatter};

#[derive(Debug, Clone)]
pub struct Entry {
    key: String,
    value: String,
}

impl Entry {
    pub fn new(key: String, value: String) -> Self {
        Entry { key, value }
    }

    pub fn to_string(&self) -> String {
        let Self { key, value } = self;
        format!("{} => {}\n", key, value)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let Self { key, value } = self;

        let mut buffer = Vec::with_capacity(self.key.len() + self.value.len() + 1);
        buffer.append(&mut key.clone().into_bytes());
        buffer.append(&mut value.clone().into_bytes());
        buffer.push(10);
        buffer
    }
}

impl PartialEq<Self> for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self.value == other.value
    }
}

impl PartialEq<String> for Entry {
    fn eq(&self, other: &String) -> bool {
        &self.key == other
    }
}

impl Display for Entry {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{} {}", self.key, self.value)
    }
}
