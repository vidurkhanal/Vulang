use nom::{combinator::eof, multi::many_till, IResult};

use crate::instruction;

use super::instruction_parser::{instruction_one, AssemblerInstruction};

#[derive(Debug, PartialEq)]
pub struct Program {
    instructions: Vec<AssemblerInstruction>,
}

pub fn program(input: &str) -> IResult<&str, Program> {
    let (input, (instructions, _)) = many_till(instruction_one, eof)(input)?;
    Ok((input, Program { instructions }))
}

impl Program {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut programs = Vec::<u8>::new();

        for instruction in &self.instructions {
            programs.append(&mut instruction.to_bytes());
        }

        programs
    }
}

#[cfg(test)]
mod program_test {
    use super::*;

    #[test]
    fn test_parse_program() {
        let result = program("load $0 #100\nload $1 #20\n");
        assert_eq!(result.is_ok(), true);
        let (leftover, p) = result.unwrap();
        assert_eq!(leftover, "");
        assert_eq!(2, p.instructions.len());
    }

    #[test]
    fn test_program_to_bytes() {
        let result = program("load $0 #100\n");
        assert_eq!(result.is_ok(), true);
        let (_, program) = result.unwrap();
        println!("{:?}", program);
        let bytecode = program.to_bytes();
        assert_eq!(bytecode.len(), 4);
        println!("{:?}", bytecode);
    }
}
