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
        let result = instruction_one("load $0 #100 \n");
        assert!(result.is_ok());
        let (_, assembler_token) = result.unwrap();
        assert_eq!(
            assembler_token,
            AssemblerInstruction {
                opcode: Token::Op { code: OpCode::LOAD },
                operand1: Some(Token::Register { reg_num: 0 }),
                operand2: Some(Token::IntegerOperand { value: 100 }),
                operand3: None
            }
        )
    }
}
