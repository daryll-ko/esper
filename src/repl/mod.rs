use crate::assembler::program_parsers::program;
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
            print!("esper > ");
            io::stdout().flush().expect("Unable to flush stdout...");
            stdin
                .read_line(&mut buffer)
                .expect("Unable to read a line from the user!");
            let buffer = buffer.trim();
            self.command_buffer.push(buffer.to_string());
            match buffer {
                ".program" => {
                    println!("Here are the instructions currently in the virtual machine:");
                    for instruction in &self.vm.program {
                        println!("{}", instruction);
                    }
                    println!("--- End of listing ---");
                }
                ".registers" => {
                    println!("Here are the registers' contents:");
                    println!("{:#?}", self.vm.registers);
                    println!("--- End of listing ---");
                }
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
                    let program = match program(buffer) {
                        Ok((_, program)) => program,
                        Err(_) => {
                            println!("Sorry, what is it you wanted to say?");
                            continue;
                        }
                    };
                    self.vm.program.append(&mut program.to_bytes());
                    self.vm.run();
                }
            }
        }
    }
}
