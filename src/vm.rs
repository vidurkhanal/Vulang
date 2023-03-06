use crate::OpCode;

pub struct VM {
    pub registers: [i32; 32],
    pc: usize,
    pub program: Vec<u8>,
    remainder: u32,
}

impl VM {
    pub fn new() -> Self {
        Self {
            registers: [0; 32],
            pc: 0,
            program: vec![],
            remainder: 0,
        }
    }

    pub fn run(&mut self) {
        let mut is_complete = false;

        while !is_complete {
            is_complete = self.execute_instruction();
        }
    }

    pub fn run_once(&mut self) {
        self.execute_instruction();
    }

    fn execute_instruction(&mut self) -> bool {
        if self.pc >= self.program.len() {
            return false;
        }

        match self.decode_op() {
            OpCode::HLT => {
                println!("HLT Encountered");
                return false;
            }
            OpCode::LOAD => {
                let register = self.next_8_bits();
                let number = self.next_16_bits();
                self.registers[register as usize] = number as i32;
            }
            OpCode::ADD => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 + register2;
            }
            OpCode::SUB => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 - register2;
            }
            OpCode::MUL => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 * register2;
            }
            OpCode::DIV => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 / register2;
                self.remainder = (register1 % register2) as u32;
            }
            OpCode::JMP => {
                let target = self.registers[self.next_8_bits() as usize];
                self.pc = target as usize;
            }
            OpCode::JMPF => {
                let value = self.registers[self.next_8_bits() as usize];
                self.pc += value as usize;
            }
            OpCode::JMPB => {
                let value = self.registers[self.next_8_bits() as usize];
                self.pc -= value as usize;
            }
            OpCode::EQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                let register_to_store_result = self.next_8_bits() as usize;
                if register1 == register2 {
                    self.registers[register_to_store_result as usize] = 1;
                } else {
                    self.registers[register_to_store_result as usize] = 0;
                }
            }
            OpCode::LT => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                let register_to_store_result = self.next_8_bits() as usize;
                if register1 < register2 {
                    self.registers[register_to_store_result as usize] = 1;
                } else {
                    self.registers[register_to_store_result as usize] = 0;
                }
            }
            OpCode::GT => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                let register_to_store_result = self.next_8_bits() as usize;
                if register1 > register2 {
                    self.registers[register_to_store_result as usize] = 1;
                } else {
                    self.registers[register_to_store_result as usize] = 0;
                }
            }
            OpCode::LTQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                let register_to_store_result = self.next_8_bits() as usize;
                if register1 <= register2 {
                    self.registers[register_to_store_result as usize] = 1;
                } else {
                    self.registers[register_to_store_result as usize] = 0;
                }
            }
            OpCode::GTQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                let register_to_store_result = self.next_8_bits() as usize;
                if register1 >= register2 {
                    self.registers[register_to_store_result as usize] = 1;
                } else {
                    self.registers[register_to_store_result as usize] = 0;
                }
            }

            OpCode::JEQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                if register1 == register2 {
                    let jump_value = self.registers[self.next_8_bits() as usize];
                    self.pc = jump_value as usize;
                }
            }

            _ => {
                eprintln!("Unrecognized opcode detected!!");
            }
        }

        true
    }

    fn decode_op(&mut self) -> OpCode {
        let opcode = OpCode::from(self.program[self.pc]);
        self.pc += 1;
        opcode
    }

    fn next_8_bits(&mut self) -> u8 {
        let res = self.program[self.pc];
        self.pc += 1;
        res
    }

    fn next_16_bits(&mut self) -> u16 {
        let res = ((self.program[self.pc] as u16) << 8) | self.program[self.pc + 1] as u16;
        self.pc += 2;
        res
    }

    pub fn add_byte(&mut self, byte: u8) {
        self.program.push(byte)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[0], 0);
    }

    #[test]
    fn test_opcode_hlt() {
        let mut test_vm = VM::new();
        test_vm.program = vec![1, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_igl() {
        let mut test_vm = VM::new();
        test_vm.program = vec![10, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_load() {
        let mut test_vm = VM::new();
        test_vm.program = vec![0, 0, 1, 244];
        test_vm.run_once();
        assert_eq!(test_vm.registers[0], 500);
    }

    #[test]
    fn test_opcode_add() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 1;
        test_vm.registers[1] = 2;
        test_vm.program = vec![2, 0, 1, 2];
        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 3);
    }

    #[test]

    fn test_opcode_jmpf() {
        let mut test_vm = VM::new();
        test_vm.registers[2] = 2;
        test_vm.program = vec![7, 2, 0, 0, 1];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 4);
    }

    #[test]
    fn test_opcode_eq() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 2;
        test_vm.registers[1] = 3;

        test_vm.program = vec![9, 0, 1, 2];
        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 0);
    }

    #[test]
    fn test_opcode_jeq() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 2;
        test_vm.registers[1] = 2;
        test_vm.registers[5] = 20;

        test_vm.program = vec![15, 0, 1, 5];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 20);

        test_vm.registers[6] = 2;
        test_vm.program = vec![15, 0, 1, 6];
        test_vm.pc = 0;
        test_vm.run_once();
        assert_eq!(test_vm.pc, 2);
    }
}
