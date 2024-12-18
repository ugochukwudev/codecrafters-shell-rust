#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::Command;

fn main() {
    let _commands: [&str; 3] = ["echo", "exit", "type"];
    let path_env = std::env::var("PATH").unwrap();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let trimmed_input = input.trim();

        if trimmed_input.is_empty() {
            continue;
        }

        match trimmed_input
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
            ["exit"] | ["exit", "0"] => {
                break;
            }
            ["type", command] => {
                if _commands.contains(command) {
                    println!("{} is a shell builtin", command);
                } else {
                    let path_separator = if cfg!(windows) { ";" } else { ":" };
                    let mut path_dirs = path_env.split(path_separator);

                    if let Some(path) = path_dirs
                        .find(|dir| std::fs::metadata(format!("{}/{}", dir, command)).is_ok())
                    {
                        println!("{command} is {path}/{command}");
                    } else {
                        println!("{command}: not found");
                    }
                }
            }
            command_parts => {
                // Handle external commands
                let program = command_parts[0];
                let args = &command_parts[1..];

                match Command::new(program).args(args).output() {
                    Ok(output) => {
                        // Print stdout of the command
                        if !output.stdout.is_empty() {
                            println!("{}", String::from_utf8_lossy(&output.stdout));
                        }
                        // Print stderr of the command, if any
                        if !output.stderr.is_empty() {
                            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
                        }
                    }
                    Err(e) => {
                        println!("Error executing {}: {}", program, e);
                    }
                }
            }
        }
    }
}
