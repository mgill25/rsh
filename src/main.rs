use std::io::{self, Write};

struct State {
    username: String,
}

fn init_state() -> State {
    let username = _run_command(&String::from("id -un")).trim().to_string();

    State { username: username }
}

fn main() {
    let stdin = io::stdin();
    println!("Welcome to rsh!");
    
    let state = init_state();

    loop {
        print!("{}@rsh> ", state.username);
        io::stdout().flush().unwrap();
        let mut command_str = String::new();
        match stdin.read_line(&mut command_str) {
            Ok(_) => {
                command_str = command_str.trim().to_string();
                run(&command_str);
            }
            Err(error) => println!("command not found = {}", error),
        }
    }
}

fn _run_command(command_str: &String) -> String {
    use std::process::{Command,Stdio};

    let command_split : Vec<&str> = command_str.split(' ').collect();
    let process = command_split[0];

    let child = Command::new(process)
                        .arg(command_split[1..].join(" "))
                        .stdout(Stdio::piped())
                        .spawn()
                        .expect("Failed to spawn the process");

    let output = child.wait_with_output()
                      .expect("Failed to wait on child");
    let status = output.status;
    let stderr = output.stderr;
    let stdout = output.stdout;
    if status.success() {
        return std::str::from_utf8(&stdout).unwrap().to_string();
    } else {
        println!("process exited with {:?}", status.code());
        return std::str::from_utf8(&stderr).unwrap().to_string();
    }
}

fn run(command_str: &String) {
    if command_str.len() == 0 {
        return
    }
    _run_command(command_str);
}
