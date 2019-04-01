pub mod prompt;

use std::io::{self, Write};

fn main() {
    prompt::welcome();

    loop {
        let input = row_input(">>> ");

        let commands: Vec<String> = input
            .trim()
            .split(' ')
            .filter(|command| command != &"")
            .map(|command| command.into())
            .collect();

        if commands.is_empty() {
            continue;
        }

        // 因为之前已经有判断了, 所以这里很肯定数组长度大于等于 1
        let first_command = &commands[0];
        let rest_commands = &commands[1..];

        dispatch_commands(first_command, rest_commands);
    }
}

fn row_input(prompt: &str) -> String {
    print!("{}", prompt);
    let _ = io::stdout().flush();
    let mut input_buf = String::with_capacity(100);
    let _ = io::stdin().read_line(&mut input_buf);
    input_buf
}

fn dispatch_commands(first_command: &str, rest_commands: &[String]) {
    match first_command {
        "quit" => prompt::quit(),
        "help" => prompt::help(),
        "history" => {
            // TODO
        }
        "get" => println!("Get ==> {:?}", rest_commands),
        "put" => println!("Put ==> {:?}", rest_commands),
        "delete" => println!("Delete ==> {:?}", rest_commands),
        "scan" => println!("scan ==> {:?}", rest_commands),
        _ => prompt::wrong(),
    }
}
