#[derive(Debug, PartialEq, Clone, Copy)]
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
    fn from(byte: u8) -> Self {
        match byte {
            0 => OpCode::HLT,
            1 => OpCode::IGL,
            2 => OpCode::LOAD,
            3 => OpCode::ADD,
            4 => OpCode::SUB,
            5 => OpCode::MUL,
            6 => OpCode::DIV,
            7 => OpCode::JMP,
            8 => OpCode::JMPF,
            9 => OpCode::JMPB,
            10 => OpCode::EQ,
            11 => OpCode::NEQ,
            12 => OpCode::GT,
            13 => OpCode::LT,
            14 => OpCode::GTQ,
            15 => OpCode::LTQ,
            16 => OpCode::JEQ,
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
