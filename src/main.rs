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

fn run_command(input: &str) {
    let args: Vec<&str> = input.split_whitespace().collect();
    let command = args[0];
    let args = &args[1..];

    match command {
        "exit" => std::process::exit(args.get(0).unwrap_or(&"0").parse().unwrap()),
        "echo" => println!("{}", args.join(" ")),
        _ => print!("{}: command not found\n", command),
    }
}
