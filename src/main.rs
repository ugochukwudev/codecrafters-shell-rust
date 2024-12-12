#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
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
            _ => {
                println!("{}: command not found", input.trim());
            }
        }
    }
}
