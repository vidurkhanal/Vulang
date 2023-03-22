pub mod assembler;
pub mod instruction;
pub mod repl;
pub mod vm;

// #[macro_use]
extern crate nom;

pub use instruction::{Instruction, OpCode};
pub use vm::VM;

fn main() {
    let mut repl = repl::REPL::new();
    repl.run();
}
