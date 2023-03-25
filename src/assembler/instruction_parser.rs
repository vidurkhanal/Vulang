use super::{
    opcode_parser::opcode_load, operand_parser::integer_operand, register_parser::register, Token,
};
use nom::IResult;

#[derive(Debug, PartialEq)]
pub struct AssemblerInstruction {
    opcode: Token,
    operand1: Option<Token>,
    operand2: Option<Token>,
    operand3: Option<Token>,
}

impl AssemblerInstruction {
    fn extract_operand(t: &Token, results: &mut Vec<u8>) {
        match t {
            Token::Register { reg_num } => results.push(*reg_num),
            Token::IntegerOperand { value } => {
                let converted = *value as u16;
                let byte1 = converted;
                let byte2 = converted >> 8;
                results.push(byte2 as u8);
                results.push(byte1 as u8);
            }
            _ => {
                eprintln!("No Operand found");
                std::process::exit(1)
            }
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut results: Vec<u8> = Vec::new();
        match self.opcode {
            Token::Op { code } => results.push(code as u8),
            _ => {
                eprintln!("Invalid OpCode provided.");
                std::process::exit(1)
            }
        }

        for t in [&self.operand1, &self.operand2, &self.operand3]
            .iter()
            .copied()
            .flatten()
        {
            // if let Some(t) = operand {
            AssemblerInstruction::extract_operand(t, &mut results)
            // }
        }

        results
    }
}

pub fn instruction_one(input: &str) -> IResult<&str, AssemblerInstruction> {
    let (input, o) = opcode_load(input)?;
    let (input, r) = register(input)?;
    let (input, i) = integer_operand(input)?;

    Ok((
        input,
        AssemblerInstruction {
            opcode: o,
            operand1: Some(r),
            operand2: Some(i),
            operand3: None,
        },
    ))
}

#[cfg(test)]
mod test_parser {
    use crate::OpCode;

    use super::*;

    #[test]
    fn instruction_one_test() {
        let result = instruction_one("load $1 #100\n");
        assert!(result.is_ok());
        let (_, assembler_token) = result.unwrap();
        assert_eq!(
            assembler_token,
            AssemblerInstruction {
                opcode: Token::Op { code: OpCode::LOAD },
                operand1: Some(Token::Register { reg_num: 1 }),
                operand2: Some(Token::IntegerOperand { value: 100 }),
                operand3: None
            }
        )
    }
}
