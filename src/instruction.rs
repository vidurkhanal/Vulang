#[derive(Debug, PartialEq)]
pub enum OpCode {
    HLT,
    IGL,
    LOAD,
    ADD,
    SUB,
    MUL,
    DIV,
    JMP,
    JMPF,
    JMPB,
    EQ,
    NEQ,
    GT,
    LT,
    GTQ,
    LTQ,
    JEQ,
}

impl From<u8> for OpCode {
    fn from(value: u8) -> Self {
        match value {
            0 => OpCode::LOAD,
            1 => OpCode::HLT,
            2 => OpCode::ADD,
            3 => OpCode::SUB,
            4 => OpCode::MUL,
            5 => OpCode::DIV,
            6 => OpCode::JMP,
            7 => OpCode::JMPF,
            8 => OpCode::JMPB,
            9 => OpCode::EQ,
            11 => OpCode::GT,
            12 => OpCode::LT,
            13 => OpCode::GTQ,
            14 => OpCode::LTQ,
            15 => OpCode::JEQ,
            _ => OpCode::IGL,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode: OpCode,
}

impl Instruction {
    pub fn new(opcode: OpCode) -> Self {
        Self { opcode }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_instruction() {
        let test_instruction = Instruction::new(OpCode::HLT);
        assert_eq!(test_instruction.opcode, OpCode::HLT);
    }
}
