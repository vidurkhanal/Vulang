pub mod opcode_parser;
pub mod register_parser;

use crate::OpCode;

#[derive(Debug, PartialEq)]
pub enum Token {
    Op { code: OpCode },
    Register { reg_num: u8 },
}
