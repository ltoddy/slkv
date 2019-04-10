pub mod commands;
pub mod communicate;
pub mod config;
pub mod prompt;

use std::io::{self, Write};
use std::str;

use commands::Commander;
use prompt::{welcome, Helper};

fn main() {
    welcome();

    loop {
        let input: String = row_input(">>> ");
        let input: &str = input.trim();
        if input.is_empty() {
            continue;
        }

        let (first_command, rest_commands) = parse_to_commands(input);
        match dispatch_command(first_command.as_str(), rest_commands) {
            Ok(_) => (),
            Err(err) => println!("Oops, {}", err),
        }
    }
}

fn row_input(prompt: &str) -> String {
    print!("{}", prompt);
    let _ = io::stdout().flush();
    let mut input_buf = String::with_capacity(129);
    let _ = io::stdin().read_line(&mut input_buf);
    input_buf
}

fn parse_to_commands(input: &str) -> (String, Vec<String>) {
    let commands: Vec<String> = input
        .split(' ')
        .filter(|command| command != &"")
        .map(|command| command.into())
        .collect();

    let first_command = &commands[0];
    let rest_commands = &commands[1..];
    (first_command.clone(), rest_commands.to_vec())
}

fn dispatch_command(command: &str, args: Vec<String>) -> Result<(), &'static str> {
    let mut commander = Commander::new();
    let helper = Helper::new();

    match command {
        "get" => commander.get(args)?,
        "put" => commander.put(args)?,
        "delete" => commander.delete(args)?,
        "scan" => commander.scan(args)?,
        "help" => helper.help(args),
        "quit" => helper.quit(),
        _ => helper.wrong(),
    };

    Ok(())
}

#[cfg(test)]
pub mod test {
    use super::parse_to_commands;

    #[test]
    pub fn test_parse_to_commands() {
        let input = "put    foo  bar";
        let (first_command, rest_commands) = parse_to_commands(input);

        assert_eq!(first_command, "put");
        assert_eq!(rest_commands, vec!["foo", "bar"]);

        let input = "put    k1  v1  k2  v2";
        let (first_command, rest_commands) = parse_to_commands(input);

        assert_eq!(first_command, "put");
        assert_eq!(rest_commands, vec!["k1", "v1", "k2", "v2"]);

        let input = "any    k1  v1  k2  v2";
        let (first_command, rest_commands) = parse_to_commands(input);

        assert_eq!(first_command, "any");
        assert_eq!(rest_commands, vec!["k1", "v1", "k2", "v2"]);
    }
}
