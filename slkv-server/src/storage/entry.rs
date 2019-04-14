use std::fmt::{Display, Error, Formatter};

#[derive(Debug, Clone)]
pub struct Entry {
    key: String,
    value: String,
}

// base
impl Entry {
    #[inline]
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
        buffer.push(32); // ascii: space
        buffer.append(&mut value.clone().into_bytes());
        buffer.push(10); // ascii: \n

        // -> b"key value\n"
        buffer
    }
}

impl PartialEq<Self> for Entry {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self.value == other.value
    }
}

// 对于key相同,那么就认同是相等,也就是说hash-code相同(因为目前的key都是String类型)
impl PartialEq<String> for Entry {
    #[inline]
    fn eq(&self, other: &String) -> bool {
        &self.key == other
    }
}

impl Display for Entry {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{} {}", self.key, self.value)
    }
}

#[cfg(test)]
pub mod test {
    use super::Entry;

    #[test]
    pub fn test_entry_struct_stringify() {
        let entry = Entry::new("hello".into(), "world".into());
        assert_eq!("hello => world\n", entry.to_string());
    }

    #[test]
    pub fn test_entry_struct_bytes() {
        let entry = Entry::new("hello".into(), "world".into());

        assert_eq!(entry.as_bytes(), b"hello world\n");
    }

    #[test]
    pub fn test_entry_struct_compare_others() {
        let entry1 = Entry::new("hello".into(), "world".into());
        let entry2 = Entry::new("hello".into(), "world".into());

        assert_eq!(entry1, entry2);
        assert_eq!(entry1, "hello".to_string());
        assert_ne!(entry1, "some".to_string());
    }
}
