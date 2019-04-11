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
        // 数据流非常简单, 获得用户输入,解析,然后分发命令.
        let input: String = row_input(">>> ");
        let input: &str = input.trim();
        if input.is_empty() {
            continue;
        }

        let (first_command, rest_commands) = parse_to_commands(input);

        if let Err(err) = dispatch_command(first_command.as_str(), rest_commands) {
            println!("Oops, {}", err)
        }
    }
}

// 仿照Python2的row_input函数.
fn row_input(prompt: &str) -> String {
    print!("{}", prompt);
    let _ = io::stdout().flush();
    let mut input_buf = String::with_capacity(129);
    let _ = io::stdin().read_line(&mut input_buf);
    input_buf
}

// 解析用户的输入. e.g. "put foo bar" => ("put", ["foo", "bar"])
// 格式: command arg1 arg2 ...
// 结果: (command, args)
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

// 分发命令, 因为每个命令要把数据发送到server端, 返回一个Result,在具体每个命令中把error变换成字符串,
// 然后由上层打印到终端里,告知用户.
// 不过返回值: Result<(), &'static str>, 这样设计其实是不好的目前,最好是类似
// Result<Response, &'static str>, 然后上层去对这个Response做处理.
// 不过我目前还没有想好如何设计这个Response.
// struct Response {
//     data: String
// }
// 这种就太过简陋了.
// 现在, 数据的处理(正常的数据),是在每个子命令下处理的,而错误的处理是在上层调用方处理的, 不统一, TODO
// 目前来说不好做,由于目前没有调用任何第三方库,没有使用类似protobuf的这样的序列化协议, 目前使用的传输协议
// 过于简陋,不方便反序列化成自定义的数据结构. (但是我目前还不想引入任何第三方的库)
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
