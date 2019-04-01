use std::process;

pub fn welcome() {
    println!("Welcome!");
    println!("  `slkv` is a simple, lightweight key-value type database.(also is a toy.)");
    println!("Have fun!");
    println!();
}

pub fn quit() {
    println!("Good bye.");
    process::exit(0);
}

pub fn help() {
    println!("Usage:\n");
    // TODO
}

pub fn get() {
    // TODO
}

pub fn put() {
    // TODO
}

pub fn delete() {
    // TODO
}

pub fn scan() {
    // TODO
}

pub fn wrong() {
    println!("Wrong command.");
    println!("  you can type the `help` command to learn more usage.");
}
