pub mod opcode_parser;
pub mod operand_parser;
pub mod register_parser;

use crate::OpCode;

#[derive(Debug, PartialEq)]
pub enum Token {
    Op { code: OpCode },
    Register { reg_num: u8 },
    IntegerOperand { value: i32 },
}
