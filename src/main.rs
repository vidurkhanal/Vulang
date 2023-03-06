pub mod instruction;
pub mod repl;
pub mod vm;

pub use instruction::{Instruction, OpCode};
pub use vm::VM;

fn main() {
    let mut repl = repl::REPL::new();
    repl.run();
}
