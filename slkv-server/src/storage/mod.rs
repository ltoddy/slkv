pub mod entry;

use std::collections::{HashMap, LinkedList};
use std::fs::{self, File};
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;

use self::entry::Entry;
use crate::FILE_PATH;

// database handler
// server端调度的核心,接受参数,生成自定义的数据结构,以及持久化.
#[derive(Debug)]
pub struct Storage {
    // HashMap 无序, LinkedList 有序.
    // 数据结构常驻内存,方便CRUD,也可以减少io.
    list: LinkedList<Entry>,
    map: HashMap<String, String>,
}

impl Storage {
    pub fn new() -> Self {
        Storage {
            list: LinkedList::new(),
            map: HashMap::new(),
        }
    }

    fn insert(&mut self, key: String, value: String) {
        self.map.insert(key.clone(), value.clone());
        self.list.push_back(Entry::new(key.clone(), value.clone()));
    }

    // 仿照Python
    pub fn load_from_file(filename: &Path) -> io::Result<Self> {
        let mut storage = Self::new();
        let file = File::open(filename)?;
        let reader = BufReader::new(&file);

        for line in reader.lines() {
            let line = line.unwrap();
            let kv: Vec<&str> = line.split(' ').collect();
            let key = kv[0].to_string();
            let value = kv[1].to_string();
            storage.insert(key, value);
        }

        Ok(storage)
    }

    pub fn dump_to_file(&self, filename: &Path) -> io::Result<()> {
        fs::write(filename, self.to_string())?;
        Ok(())
    }

    fn to_string(&self) -> String {
        let mut buffer = String::with_capacity(1024);

        self.list
            .iter()
            .for_each(|entry| buffer.push_str(format!("{}\n", entry).as_str()));

        buffer
    }

    pub fn get(&self, keys: Vec<String>) -> String {
        // log
        println!("get request, received data: {:?}", keys);

        let mut data: String = String::with_capacity(1024);

        // 考虑到传过来的key未必存在,所以给予默认的value: "None"
        keys.iter().for_each(|key| {
            data.push_str(
                format!(
                    "{} => {}\n",
                    key,
                    self.map.get(key).unwrap_or(&String::from("None"))
                )
                .as_str(),
            )
        });

        data
    }

    pub fn put(&mut self, mut kvs: Vec<String>) -> String {
        // log
        println!("put request, received data: {:?}", kvs);

        // 如果传过来的参数是奇数个,例如:  key1 => value1, key2 => value2, key3
        // 那么忽略最后一个参数.
        if kvs.len() % 2 != 0 {
            kvs.pop(); // throw the last useless element
        }

        // 上一步已经保证了参数个数为偶数个,这里可以通过步长为2的方式去遍历key与value.
        for index in (0..kvs.len()).step_by(2) {
            let key = kvs[index].to_string();
            let value = kvs[index + 1].to_string();

            if self.map.contains_key(&key) {
                // key如果存在,则更新
                self.map.insert(key.clone(), value.clone());
                self.list = self
                    .list
                    .iter()
                    .filter(|entry| *entry != &key)
                    .cloned()
                    .collect::<LinkedList<Entry>>(); // 说实话, rust的LinkedList实在太...
                                                     // ugly code
                self.list.push_back(Entry::new(key.clone(), value.clone()));
            } else {
                // key如果不存在,则插入到最后
                self.list.push_back(Entry::new(key.clone(), value.clone()));
                self.map.insert(key.clone(), value.clone());
            }
        }

        // 这里的io操作没有开新的线程去执行io操作.
        // 如果持久化操作失败,就停机.
        // 虽 spawn 会返回包含 ThreadHandler 勾柄的Result, 以及 join 也会返回一个Result
        // 如果出现异常,数据丢失,我目前来说还没想到什么好的解决方案.
        self.dump_to_file(Path::new(FILE_PATH))
            .expect("Fatal error, shutdown!");

        String::from("Ok\n")
    }

    pub fn delete(&mut self, keys: Vec<String>) -> String {
        // log
        println!("delete request, received data: {:?}", keys);

        keys.iter().for_each(|key| {
            self.map.remove(key);
            self.list = self
                .list
                .iter()
                .filter(|entry| entry != &key)
                .cloned()
                .collect::<LinkedList<Entry>>();
        });

        self.dump_to_file(Path::new(FILE_PATH))
            .expect("Fatal error, shutdown!");

        String::from("Ok\n")
    }

    pub fn scan(&self, args: Vec<String>) -> String {
        // log
        println!("scan request, received data: {:?}", args);

        let (start, end) = (
            // 虽然客户端会校验参数的正确性,但是服务端给予更多的包容.
            // 当解析参数错误的时候,就赋予他默认值
            args[0].parse::<usize>().unwrap_or(0),
            args[1].parse::<usize>().unwrap_or_else(|_| self.list.len()),
        );

        // 截取
        self.list
            .iter()
            .skip(start)
            .take(end - start)
            .map(Entry::to_string)
            .fold(String::with_capacity(1024), |mut acc, x| {
                acc.push_str(x.as_str());
                acc
            })
    }
}

impl PartialEq for Storage {
    // Just for test conveniently.
    fn eq(&self, other: &Self) -> bool {
        self.map == other.map && self.list == other.list
    }
}

impl Default for Storage {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
pub mod test {
    use super::Storage;
    use std::collections::linked_list::LinkedList;
    use std::collections::HashMap;

    #[test]
    fn test_storage_stringify() {
        let mut storage = Storage::new();

        storage.insert("k1".to_string(), "v1".to_string());
        storage.insert("k2".to_string(), "v2".to_string());

        assert_eq!("k1 v1\nk2 v2\n", storage.to_string());
    }

    #[test]
    fn test_storage_get_command() {
        let mut storage = Storage::new();

        let res = storage.get(vec!["non-exist".to_owned()]);
        assert_eq!(res, "non-exist => None\n");

        storage.put(vec!["k1".to_owned(), "v1".to_owned()]);
        let res = storage.get(vec!["k1".to_owned(), "non-exist".to_owned()]);
        assert_eq!(res, "k1 => v1\nnon-exist => None\n");
    }

    #[test]
    fn test_storage_put_command() {
        use super::Entry;

        let mut storage = Storage::new();

        storage.put(vec![
            "k1".to_string(),
            "v1".to_string(),
            "k2".to_string(),
            "v2".to_string(),
            "k3".to_string(),
            "v3".to_string(),
        ]);

        let mut temp_map = HashMap::new();
        let mut temp_list = LinkedList::new();

        temp_map.insert("k1".to_string(), "v1".to_string());
        temp_map.insert("k2".to_string(), "v2".to_string());
        temp_map.insert("k3".to_string(), "v3".to_string());

        temp_list.push_back(Entry::new("k1".to_string(), "v1".to_string()));
        temp_list.push_back(Entry::new("k2".to_string(), "v2".to_string()));
        temp_list.push_back(Entry::new("k3".to_string(), "v3".to_string()));

        let temp_storage = Storage {
            map: temp_map,
            list: temp_list,
        };

        assert_eq!(storage, temp_storage);
    }

    #[test]
    fn test_storage_delete_command() {
        use super::Entry;

        let mut storage = Storage::new();

        storage.put(vec![
            "k1".to_string(),
            "v1".to_string(),
            "k2".to_string(),
            "v2".to_string(),
            "k3".to_string(),
            "v3".to_string(),
        ]);

        let mut temp_map = HashMap::new();
        let mut temp_list = LinkedList::new();

        temp_map.insert("k1".to_string(), "v1".to_string());
        temp_map.insert("k2".to_string(), "v2".to_string());
        temp_map.insert("k3".to_string(), "v3".to_string());

        temp_list.push_back(Entry::new("k1".to_string(), "v1".to_string()));
        temp_list.push_back(Entry::new("k2".to_string(), "v2".to_string()));
        temp_list.push_back(Entry::new("k3".to_string(), "v3".to_string()));

        let temp_storage = Storage {
            map: temp_map,
            list: temp_list,
        };

        storage.delete(vec!["non-exist".to_owned()]);

        assert_eq!(storage, temp_storage);

        // ---------

        let mut storage = Storage::new();

        storage.put(vec![
            "k1".to_string(),
            "v1".to_string(),
            "k2".to_string(),
            "v2".to_string(),
            "k3".to_string(),
            "v3".to_string(),
        ]);

        let mut temp_map = HashMap::new();
        let mut temp_list = LinkedList::new();

        temp_map.insert("k1".to_string(), "v1".to_string());
        temp_map.insert("k3".to_string(), "v3".to_string());

        temp_list.push_back(Entry::new("k1".to_string(), "v1".to_string()));
        temp_list.push_back(Entry::new("k3".to_string(), "v3".to_string()));

        let temp_storage = Storage {
            map: temp_map,
            list: temp_list,
        };

        storage.delete(vec!["k2".to_owned()]);

        assert_eq!(storage, temp_storage);
    }

    #[test]
    fn test_storage_scan_command() {
        let mut storage = Storage::new();

        storage.put(vec![
            "k1".to_string(),
            "v1".to_string(),
            "k2".to_string(),
            "v2".to_string(),
            "k3".to_string(),
            "v3".to_string(),
        ]);

        let response = storage.scan(vec!["0".to_owned(), "10".to_owned()]);
        assert_eq!(response, "k1 => v1\nk2 => v2\nk3 => v3\n");

        let response = storage.scan(vec!["wrong_param".to_owned(), "wrong_param".to_owned()]);
        assert_eq!(response, "k1 => v1\nk2 => v2\nk3 => v3\n");
    }
}
