pub mod entry;

use std::collections::{HashMap, LinkedList};
use std::fs::{self, File};
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;

use self::entry::Entry;
use crate::FILE_PATH;

#[derive(Debug)]
pub struct Storage {
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

        self.list.iter().for_each(|entry| {
            buffer.push_str(format!("{}\n", entry).as_str());
        });

        buffer
    }

    pub fn get(&self, keys: Vec<String>) -> String {
        println!("get request, received data: {:?}", keys);

        let mut data: String = String::with_capacity(1024);

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
        println!("put request, received data: {:?}", kvs);

        if kvs.len() % 2 != 0 {
            kvs.pop(); // throw the last useless element
        }

        for index in (0..kvs.len()).step_by(2) {
            let key = kvs[index].to_string();
            let value = kvs[index + 1].to_string();

            if self.map.contains_key(&key) {
                self.map.insert(key.clone(), value.clone());
                self.list = self
                    .list
                    .iter()
                    .filter(|entry| *entry != &key)
                    .cloned()
                    .collect::<LinkedList<Entry>>();
                self.list.push_back(Entry::new(key.clone(), value.clone()));
            } else {
                self.list.push_back(Entry::new(key.clone(), value.clone()));
                self.map.insert(key.clone(), value.clone());
            }
        }

        self.dump_to_file(Path::new(FILE_PATH))
            .expect("Fatal error, shutdown!");

        String::from("Ok\n")
    }

    pub fn delete(&mut self, keys: Vec<String>) -> String {
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
        println!("scan request, received data: {:?}", args);

        let (start, end) = (
            args[0].parse::<usize>().unwrap_or(0),
            args[1].parse::<usize>().unwrap_or(0),
        );

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

impl Default for Storage {
    fn default() -> Self {
        Self::new()
    }
}
