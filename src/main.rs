use std::io::{self, Write};
use std::process::Command;

fn main() {
    let stdin = io::stdin();    // TODO: "concurrent reads of Stdin must be executed with care" ?? need to know more
    println!("Welcome to rsh!");
    print!("rsh > ");
    io::stdout().flush().unwrap();
    loop {
        let mut command_str = String::new();
        match stdin.read_line(&mut command_str) {
            Ok(_) => {
                command_str = command_str.trim().to_string();
                interpret(&command_str);
                print!("rsh> ");
                io::stdout().flush().unwrap();
            }
            Err(error) => println!("command not found = {}", error),
        }
    }
}

fn interpret(command_str: &String) {
    // print!("We got a command to interpret = {}", command)
    // can we split up the command string into an array/list of sub-strings?
    if command_str.len() == 0 {
        return
    }
    let command_split : Vec<&str> = command_str.split(' ').collect();
    // println!("{:?}", command_split);
    let process = command_split[0];
    println!("we must execute {}", process);
    
    // TODO: Can we directly work with syscalls somehow?
    let output = Command::new(process)
                        .arg(command_split[1..].join(" "))
                        .spawn()
                        .expect("Failed to execute command");
    println!("{:?}", output);
}
