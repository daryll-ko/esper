use crate::vm::VM;
use std;
use std::io;
use std::io::Write;

pub struct REPL {
    command_buffer: Vec<String>,
    vm: VM,
}

impl REPL {
    pub fn new() -> REPL {
        REPL {
            command_buffer: vec![],
            vm: VM::new(),
        }
    }
    pub fn run(&mut self) {
        println!("Activating esper powers...");
        loop {
            let mut buffer = String::new();
            let stdin = io::stdin();
            print!("> ");
            io::stdout().flush().expect("Unable to flush stdout...");
            stdin
                .read_line(&mut buffer)
                .expect("Unable to read a line from the user!");
            let buffer = buffer.trim();
            self.command_buffer.push(buffer.to_string());
            match buffer {
                ".history" => {
                    for command in &self.command_buffer {
                        println!("{}", command);
                    }
                }
                ".quit" => {
                    println!("Deactivating esper powers...");
                    std::process::exit(0);
                }
                _ => {
                    println!("Invalid input!");
                }
            }
        }
    }
}
