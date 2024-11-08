#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
    
        stdin.read_line(&mut input).unwrap();

        run_command(&input);
    }
}

const BUILTINS: &[&str] = &["exit", "echo", "type"];

fn run_command(input: &str) {
    let args: Vec<&str> = input.split_whitespace().collect();
    let command = args[0];
    let args = &args[1..];

    if BUILTINS.contains(&command) {
        run_builtin(command, args);
    } else {
        println!("{}: command not found", command);
    }
}

fn run_builtin(command: &str, args: &[&str]) {
    match command {
        "exit" => std::process::exit(args.get(0).unwrap_or(&"0").parse().unwrap()),
        "echo" => println!("{}", args.join(" ")),
        "type" => run_type(args.get(0).unwrap_or(&"")),
        _ => println!("{}: command not found", command),
    }
}

fn run_type(command: &str) {
    if BUILTINS.contains(&command) {
        println!("{} is a shell builtin", command);
    } else {
        find_command_in_path(command);
    }
}
 
fn find_command_in_path(command: &str) {
    let path = std::env::var("PATH").unwrap();
    for dir in path.split(':') {
        let full_path = format!("{}/{}", dir, command);
        if std::path::Path::new(&full_path).exists() {
            println!("{}: {}", command, full_path);
            return;
        }
    }
    println!("{}: not found", command);
}