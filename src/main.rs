#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    let _commands: [&str; 3] = ["echo", "exit", "type"];
    let path_env = std::env::var("PATH").unwrap();
    // Uncomment this block to pass the first stage
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        match input
            .trim()
            .to_lowercase()
            .as_str()
            .split_whitespace()
            .collect::<Vec<_>>()
            .as_slice()
        {
            ["echo", rest @ ..] => {
                if !rest.is_empty() {
                    println!("{}", rest.join(" "));
                } else {
                    println!("No additional arguments provided for echo.");
                }
            }
            ["exit", "0"] => {
                break;
            }
            ["type", rest] => {
                if _commands.contains(rest) {
                    println!("{} is a shell builtin", rest);
                } else {
                    let split = &mut path_env.split(':');
                    if let Some(path) =
                        split.find(|path| std::fs::metadata(format!("{}/{}", path, rest)).is_ok())
                    {
                        println!("{rest} is {path}/{rest}");
                    } else {
                        println!("{rest} not found");
                    }
                }
            }
            _ => {
                println!("{}: command not found", input.trim());
            }
        }
    }
}
