pub mod prompt;

use std::io;
use std::io::Write;
use std::process;

const LIMIT_HISTORY: usize = 1000;

fn main() {
    prompt::welcome();
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut history_commands = Vec::with_capacity(LIMIT_HISTORY + 1);

    loop {
        print!(">>> ");
        let _ = stdout.flush();

        let mut input = String::with_capacity(100);
        let _ = stdin.read_line(&mut input).map_err(|_| {
            println!("命令格式出错");
            prompt::help();
        });

        history_commands.push(input.clone());
        if history_commands.len() > LIMIT_HISTORY {
            history_commands.remove(0);
        }

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

        dispatch(first_command, rest_commands);
    }
}

fn dispatch(first_command: &str, rest_commands: &[String]) {
    if first_command == "quit" {
        println!("Good bye.");
        process::exit(0);
    } else if first_command == "help" {
        prompt::help();
    } else if first_command == "history" {
        // TODO
    } else if first_command == "get" {
        println!("Get ==> {:?}", rest_commands);
    } else if first_command == "put" {
        println!("Put ==> {:?}", rest_commands);
    } else if first_command == "delete" {
        println!("Delete ==> {:?}", rest_commands);
    } else if first_command == "scan" {
        println!("Scan ==> {:?}", rest_commands);
    } else {
        println!("Wrong command.");
        println!("  you can type the `help` command to learn more usage.");
    }
}
