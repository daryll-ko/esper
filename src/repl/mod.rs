use crate::vm::VM;
use std;
use std::io;
use std::io::Write;
use std::num::ParseIntError;

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
    fn parse_hex(&mut self, input: &str) -> Result<Vec<u8>, ParseIntError> {
        let tokens = input.split(" ").collect::<Vec<&str>>();
        let mut results: Vec<u8> = vec![];
        for hex_string in tokens {
            let byte = u8::from_str_radix(&hex_string, 16);
            match byte {
                Ok(result) => {
                    results.push(result);
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(results)
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
                    let results = self.parse_hex(buffer);
                    match results {
                        Ok(bytes) => {
                            for byte in bytes {
                                self.vm.add_byte(byte);
                            }
                        }
                        Err(_e) => {
                            println!("Unable to decode hex string. Please enter 4 groups of 2 hex characters.");
                        }
                    }
					self.vm.run();
                }
            }
        }
    }
}