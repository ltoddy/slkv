pub mod entry;

use std::collections::{HashMap, LinkedList};
use std::fs::{self, File};
use std::io;
use std::io::{Read, Write};
use std::path::Path;

use self::entry::Entry;

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

    pub fn load_from_file(filename: &Path) -> io::Result<Self> {
        let mut file = File::open(filename)?;
        let mut buffer = String::with_capacity(1024);
        file.read_to_string(&mut buffer)?;

        let mut storage = Self::new();

        let a = buffer.split("\n");
        println!("=====> {:?}", a);

        Ok(storage)
    }

    pub fn dump_to_file(&self, filename: &Path) -> io::Result<()> {
        //        let mut file = match File::open(filename) {
        //            Ok(file) => file,
        //            Err(_) => File::create(filename).map_err(|err| {
        //                eprintln!("Can't create: {}", err);
        //                err
        //            })?,
        //        };
        fs::write(filename, self.to_string())?;

        Ok(())
    }

    fn as_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::with_capacity(1024);

        self.list
            .iter()
            .for_each(|entry| buffer.append(&mut entry.as_bytes()));

        buffer
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

        println!("get final:====> {:?}", self);

        data
    }

    pub fn put(&mut self, mut kvs: Vec<String>) -> String {
        println!("put request, received data: {:?}", kvs);

        if kvs.len() % 2 != 0 {
            kvs.pop(); // throw away the last useless element
        }

        for index in (0..kvs.len()).step_by(2) {
            let key = kvs[index].to_string();
            let value = kvs[index + 1].to_string();
            self.list.push_back(Entry::new(key.clone(), value.clone()));
            self.map.insert(key.clone(), value.clone());
        }

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
                .map(|entry| entry.clone())
                .collect::<LinkedList<Entry>>();
        });

        String::from("Ok\n")
    }

    // TODO
    // pub fn scan(&self, start: usize, end: usize) {}
}
