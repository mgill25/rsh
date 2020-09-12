use std::io::{self, Write};
use std::process::Command;

fn main() {
    let stdin = io::stdin();
    println!("Welcome to rsh!");
    loop {
        print!("rsh> ");
        io::stdout().flush().unwrap();
        let mut command_str = String::new();
        match stdin.read_line(&mut command_str) {
            Ok(_) => {
                command_str = command_str.trim().to_string();
                interpret(&command_str);
            }
            Err(error) => println!("command not found = {}", error),
        }
    }
}

/**
 * How can we write tests for this?
 */
fn interpret(command_str: &String) {
    if command_str.len() == 0 {
        return
    }
    let command_split : Vec<&str> = command_str.split(' ').collect();
    
    let process = command_split[0];
    
    // TODO: Can we directly work with syscalls somehow?
    let status = Command::new(process)
                        .arg(command_split[1..].join(" "))
                        .status();
    match status {
        Ok(s) => {
            println!("process exited with {}", s);
        },
        Err(error) => println!("error while spawning: {}", error)
    }
}
