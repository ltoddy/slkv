use std::process;

pub fn welcome() {
    println!("Welcome!");
    println!("  `slkv` is a simple, lightweight key-value type database.(also is a toy.)");
    println!();
    println!("You can type `help` to get usages.");
    println!("You can type `quit` or `ctrl + c` to quit.");
    println!("Have fun!");
    println!();
}

// 本可以不定义这个Helper结构体,但是如果不定义,客户端就显得太面向过程了.
pub struct Helper {}

impl Helper {
    pub fn new() -> Self {
        Helper {}
    }

    pub fn help(&self, operator: Vec<String>) {
        if operator.is_empty() {
            println!("You seem to have trouble.");
            println!("This key-value database has four operators:");
            println!("  - `get`");
            println!("  - `put`");
            println!("  - `delete`");
            println!("  - `scan`");
            return;
        }
        let operator = &operator[0];
        match operator.as_str() {
            "get" => Self::get_command_usage(),
            "put" => Self::put_command_usage(),
            "delete" => Self::delete_command_usage(),
            "scan" => Self::scan_command_usage(),
            _ => self.help(vec![]),
        }
    }

    fn get_command_usage() {
        println!("`Get` command:");
        println!("\te.g.");
        println!("\t- get key");
        println!("\t- get key1 key2 ...");
        println!("tips: accept any number of parameters.");
    }

    fn put_command_usage() {
        println!("`Put` command:");
        println!("\te.g.");
        println!("\t- put key1 value1");
        println!("\t- put key1 value1 key2 value2 ...");
        println!("tips: parameters must exists in pairs.");
    }

    fn delete_command_usage() {
        println!("`delete` command:");
        println!("\te.g.");
        println!("\t- delete key");
        println!("\t- delete key1 key2 ...");
        println!("tips: accept any number of parameters.");
    }

    fn scan_command_usage() {
        println!("`scan` command:");
        println!("\te.g.");
        println!("\t- get 0 10");
        println!("tips: accept two integer parameters, `begin` and `end`.");
    }

    pub fn quit(&self) {
        println!("Good bye.");
        process::exit(0);
    }

    pub fn wrong(&self) {
        println!("Wrong command.");
        println!("  you can type the `help` command to learn more usage.");
    }
}

// Just for clippy
impl Default for Helper {
    fn default() -> Self {
        Self::new()
    }
}
