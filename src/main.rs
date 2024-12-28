#[allow(unused_imports)]
use std::io::{self, Write};
use std::{env::args, os::unix::process, process::ExitCode};

fn main() {
    loop {
        // Uncomment this block to pass the first stage
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        // let command = input.trim();
        let trimmed = input.trim();

        // REPL: READ-EVAL-PRINT LOOP

        // To properly implement REPL: I have to implement a loop.

        let mut parts = trimmed.split_whitespace();
        let command = parts.next().unwrap_or("");

        match command {
            "echo" => {
                let args: Vec<&str> = parts.collect();
                println!("{}", args.join(" "))
            }
            "exit" => std::process::exit(0),
            "type" => {
                let command_to_check = parts.next().unwrap_or("");
                match command_to_check {
                    "exit" | "echo" | "type"  => {
                        println!("{} is a shell builtin", command_to_check)
                    }
                    _ => {
                        println!("{}: not found", command_to_check)
                    }
                }
            }
            _ => {
                println!("{}: command not found", command)
            }
        }
    }
}
