use crate::{assembler::program_parser::program, VM};
use std::{cmp::min, io::Write, num::ParseIntError};

pub struct REPL {
    command_buffer: Vec<String>,
    vm: VM,
}

impl Default for REPL {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(clippy::print_literal)]
impl REPL {
    pub fn new() -> Self {
        Self {
            command_buffer: vec![],
            vm: VM::new(),
        }
    }
    pub fn run(&mut self) {
        println!("Welcome to Vulang v{}!! [Rust] ", env!("CARGO_PKG_VERSION"),);
        let mut buffer = String::new();
        loop {
            buffer.clear();
            let stdin = std::io::stdin();

            print!(">>> ");

            if let Err(e) = std::io::stdout().flush() {
                eprintln!("Unable to flush stdout \n Error Details: {}", e);
            }

            if let Err(e) = stdin.read_line(&mut buffer) {
                eprintln!(
                    "Unable to take input from the user. \n Error Details: {}",
                    e
                );
            }

            let buffer = buffer.trim();
            self.command_buffer.push(String::from(buffer));
            match buffer {
                ":quit" => {
                    eprintln!("See you again!!");
                    std::process::exit(0);
                }
                ":clear" => {
                    println!("{}", "b\x1B[2J\x1B[1;1H");
                }
                ":history" => {
                    println!("-- Command History --");
                    let n = self.command_buffer.len();
                    for i in 0..min(n, 10_usize) {
                        println!(" -> {}", self.command_buffer[n - i - 1])
                    }
                    println!("-- END --")
                }
                ":program" => {
                    println!("-- Instructions --");
                    for instruction in &self.vm.program {
                        println!("{instruction}")
                    }
                    println!("-- END --")
                }
                ":registers" => {
                    println!("-- Registers --");
                    println!("{:?}", self.vm.registers);
                    println!("-- END --")
                }
                _ => {
                    // if let Ok(bytes) = self.parse_hex(buffer) {
                    //     for byte in bytes {
                    //         self.vm.add_byte(byte)
                    //     }
                    // } else {
                    //     eprintln!("The compiler couldn't parse your input. Please provide valid commands and arguments")
                    // };

                    // let parsed_program = program(buffer);
                    // if parsed_program.is_err() {
                    //     eprintln!("Unable to parse the input");
                    //     continue;
                    // }

                    // let (_, result) = parsed_program.unwrap();
                    // let bytecode = result.to_bytes();
                    // for byte in bytecode {
                    //     self.vm.add_byte(byte);
                    // }

                    let program = match program(buffer) {
                        Ok((_, program)) => program,
                        Err(_) => {
                            eprintln!("Unable to parse input");
                            continue;
                        }
                    };

                    self.vm.program.append(&mut program.to_bytes());

                    self.vm.run_once();
                }
            }
        }
    }

    pub fn parse_hex(&mut self, user_input: &str) -> Result<Vec<u8>, ParseIntError> {
        let split: Vec<&str> = user_input.split(' ').collect::<Vec<&str>>();
        let mut res: Vec<u8> = vec![];
        for hex_string in split {
            let byte: u8 = u8::from_str_radix(hex_string, 16)?;
            res.push(byte);
        }
        Ok(res)
    }
}
