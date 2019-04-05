pub mod commands;
pub mod communicate;
pub mod prompt;

use std::io::{self, Write};

use commands::Commander;
use prompt::{welcome, Helper};
use std::str;

fn main() {
    welcome();

    loop {
        let input: String = row_input(">>> ");

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
        let first_command: &String = &commands[0];
        let rest_commands: &[String] = &commands[1..];

        match dispatch_commands(first_command, rest_commands) {
            Ok(_) => println!("ok"),
            Err(err) => println!("Oops, {}", err),
        }
    }
}

fn row_input(prompt: &str) -> String {
    print!("{}", prompt);
    let _ = io::stdout().flush();
    let mut input_buf = String::with_capacity(100);
    let _ = io::stdin().read_line(&mut input_buf);
    input_buf
}

fn dispatch_commands(first_command: &str, rest_commands: &[String]) -> Result<(), &'static str> {
    let mut commander = Commander::new();
    let helper = Helper::new();

    match first_command {
        "get" => commander.get(rest_commands)?,
        "put" => commander.put(rest_commands)?,
        "delete" => commander.delete(rest_commands)?,
        "scan" => commander.scan(rest_commands)?,
        "help" => helper.help(rest_commands),
        "quit" => helper.quit(),
        _ => helper.wrong(),
    };

    Ok(())
}
