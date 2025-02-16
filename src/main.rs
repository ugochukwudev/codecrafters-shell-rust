#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::Command;
use std::{
    env,
    path::{self, Path},
};

fn main() {
    let _commands: [&str; 4] = ["echo", "exit", "type", "pwd"];
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

        // Split input into parts without altering the case
        let parts: Vec<&str> = trimmed_input.split_whitespace().collect();

        match parts.as_slice() {
            ["echo", rest @ ..] => {
                if !rest.is_empty() {
                    let first_char = rest[0].chars().next().unwrap();
                    if first_char == '"' || first_char == '\'' {
                        let mut quoted_string = rest.join(" ");
                        quoted_string = quoted_string.replace(first_char, "").to_string();
                        println!("{}", quoted_string);
                    } else {
                        println!("{}", rest.join(" "));
                    }
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
            ["pwd"] => match env::current_dir() {
                Ok(path) => println!("{}", path.display()),
                Err(e) => eprintln!("Error getting current working directory: {}", e),
            },
            ["cd", path] => {
                // let formatted_path = if path.starts_with('/') {
                //     &path[1..]
                // } else {
                //     path
                // };
                if *path == "~" {
                    let home = env::var("HOME").unwrap();
                    let path = Path::new(&home);
                    if let Err(_) = env::set_current_dir(&path) {
                        println!("{:?}: No such file or directory", path);
                    }
                } else if env::set_current_dir(Path::new(path)).is_ok() {
                } else {
                    println!("cd: {}: No such file or directory", path);
                }
            }
            [program, args @ ..] => {
                // Handle external commands
                match Command::new(program).args(args).output() {
                    Ok(output) => {
                        // Print stdout of the command
                        if !output.stdout.is_empty() {
                            print!("{}", String::from_utf8_lossy(&output.stdout));
                        }
                        // Print stderr of the command, if any
                        if !output.stderr.is_empty() {
                            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
                        }
                    }
                    Err(e) => {
                        println!("{}: command not found", program);
                    }
                }
            }
            _ => {
                println!("{}: command not found", trimmed_input);
            }
        }
    }
}
