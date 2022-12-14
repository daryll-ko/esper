use crate::instruction::Opcode;

pub struct VM {
    pub registers: [i32; 32],
    program_counter: usize,
    pub program: Vec<u8>,
    remainder: u32,
    comparison_flag: bool,
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            program_counter: 0,
            program: vec![],
            remainder: 0,
            comparison_flag: false,
        }
    }

    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.program_counter]);
        self.program_counter += 1;
        opcode
    }

    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.program_counter];
        self.program_counter += 1;
        result
    }

    fn next_16_bits(&mut self) -> u16 {
        let result = ((self.program[self.program_counter] as u16) << 8)
            | self.program[self.program_counter + 1] as u16;
        self.program_counter += 2;
        result
    }

    fn execute_instruction(&mut self) -> bool {
        if self.program_counter >= self.program.len() {
            return true;
        }
        match self.decode_opcode() {
            Opcode::HALT => {
                println!("HALT encountered!");
                true
            }
            Opcode::LOAD => {
                let register = self.next_8_bits() as usize;
                let number = self.next_16_bits() as i32;
                self.registers[register] = number;
                false
            }
            Opcode::ADD => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 + register2;
                false
            }
            Opcode::SUBTRACT => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 - register2;
                false
            }
            Opcode::MULTIPLY => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 * register2;
                false
            }
            Opcode::DIVIDE => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 / register2;
                self.remainder = (register1 % register2) as u32;
                false
            }
            Opcode::JUMP => {
                let target = self.registers[self.next_8_bits() as usize];
                self.program_counter = target as usize;
                false
            }
            Opcode::JUMPFORWARD => {
                let jump = self.registers[self.next_8_bits() as usize];
                self.program_counter += jump as usize;
                false
            }
            Opcode::JUMPBACKWARD => {
                let jump = self.registers[self.next_8_bits() as usize];
                self.program_counter -= jump as usize;
                false
            }
            Opcode::EQUAL => {
                self.comparison_flag = self.registers[self.next_8_bits() as usize]
                    == self.registers[self.next_8_bits() as usize];
                false
            }
            Opcode::NOTEQUAL => {
                self.comparison_flag = self.registers[self.next_8_bits() as usize]
                    != self.registers[self.next_8_bits() as usize];
                false
            }
            Opcode::GREATER => {
                self.comparison_flag = self.registers[self.next_8_bits() as usize]
                    > self.registers[self.next_8_bits() as usize];
                false
            }
            Opcode::LESS => {
                self.comparison_flag = self.registers[self.next_8_bits() as usize]
                    < self.registers[self.next_8_bits() as usize];
                false
            }
            Opcode::GREATEREQUAL => {
                self.comparison_flag = self.registers[self.next_8_bits() as usize]
                    >= self.registers[self.next_8_bits() as usize];
                false
            }
            Opcode::LESSEQUAL => {
                self.comparison_flag = self.registers[self.next_8_bits() as usize]
                    <= self.registers[self.next_8_bits() as usize];
                false
            }
            Opcode::JUMPIF => {
                let register = self.next_8_bits() as usize;
                let target = self.registers[register];
                if self.comparison_flag {
                    self.program_counter = target as usize;
                }
                false
            }
            Opcode::ILLEGAL => true,
        }
    }

    pub fn add_byte(&mut self, byte: u8) {
        self.program.push(byte);
    }

    pub fn run(&mut self) {
        let mut done = false;
        while !done {
            done = self.execute_instruction();
        }
    }
}

impl Default for VM {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vm_creation() {
        let test_vm = VM::new();
        for i in 0..32 {
            assert_eq!(test_vm.registers[i], 0);
        }
    }

    #[test]
    fn test_opcode_halt() {
        let mut test_vm = VM::new();
        test_vm.program = vec![0, 0, 0, 0];
        test_vm.run();
        assert_eq!(test_vm.program_counter, 1);
    }

    #[test]
    fn test_opcode_load() {
        let mut test_vm = VM::new();
        test_vm.program = vec![1, 0, 1, 58, 0];
        test_vm.run();
        assert_eq!(test_vm.registers[0], 314);
    }

    #[test]
    fn test_opcode_add() {
        let mut test_vm = VM::new();
        test_vm.program = vec![1, 0, 20, 22, 1, 1, 12, 34, 2, 0, 1, 2];

        // register 0: 5_142
        //          1: 3_106
        //          2: 5_142 + 3_106 = 8_248

        test_vm.run();
        assert_eq!(test_vm.registers[2], 8_248);
    }

    #[test]
    fn test_opcode_subtract() {
        let mut test_vm = VM::new();
        test_vm.program = vec![1, 0, 20, 22, 1, 1, 12, 34, 3, 0, 1, 2];

        // register 0: 5_142
        //          1: 3_106
        //          2: 5_142 - 3_106 = 2_036

        test_vm.run();
        assert_eq!(test_vm.registers[2], 2_036);
    }

    #[test]
    fn test_opcode_multiply() {
        let mut test_vm = VM::new();
        test_vm.program = vec![1, 0, 20, 22, 1, 1, 12, 34, 4, 0, 1, 2];

        // register 0: 5_142
        //          1: 3_106
        //          2: 5_142 * 3_106 = 15_971_052

        test_vm.run();
        assert_eq!(test_vm.registers[2], 15_971_052);
    }

    #[test]
    fn test_opcode_divide() {
        let mut test_vm = VM::new();
        test_vm.program = vec![1, 0, 20, 22, 1, 1, 12, 34, 5, 0, 1, 2];

        // register 0: 5_142
        //          1: 3_106
        //          2: 5_142 / 3_106 = 1 remainder 2_036

        test_vm.run();
        assert_eq!(test_vm.registers[2], 1);
        assert_eq!(test_vm.remainder, 2_036);
    }

    #[test]
    fn test_opcode_jump() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 1;
        test_vm.program = vec![6, 0, 0, 0];
        test_vm.run();
        assert_eq!(test_vm.program_counter, 2);
    }

    #[test]
    fn test_opcode_jump_forward() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 2;
        test_vm.program = vec![7, 0, 0, 0, 0, 0];

        //                     ----  o-->-->  .

        test_vm.run();
        assert_eq!(test_vm.program_counter, 5);
    }

    #[test]
    fn test_opcode_jump_backward() {
        let mut test_vm = VM::new();
        test_vm.program = vec![1, 3, 0, 1, 8, 3, 0, 3, 5, 0, 0];

        //                     ----------  ----
        //                                    <--o
        //                                    ----------     .

        test_vm.run();
        assert_eq!(test_vm.program_counter, 10);
        assert_eq!(test_vm.registers[5], -1);
    }

    #[test]
    fn test_opcode_equal() {
        let mut test_vm = VM::new();
        test_vm.program = vec![1, 0, 1, 0, 1, 3, 1, 0, 9, 0, 3];
        test_vm.run();
        assert_eq!(test_vm.comparison_flag, true);
        test_vm.program.extend([1, 3, 0, 1, 9, 0, 3]);
        test_vm.run();
        assert_eq!(test_vm.comparison_flag, false);
    }

    #[test]
    fn test_opcode_not_equal() {
        let mut test_vm = VM::new();
        test_vm.program = vec![1, 0, 1, 0, 1, 3, 1, 0, 10, 0, 3];
        test_vm.run();
        assert_eq!(test_vm.comparison_flag, false);
        test_vm.program.extend([1, 3, 0, 1, 10, 0, 3]);
        test_vm.run();
        assert_eq!(test_vm.comparison_flag, true);
    }

    #[test]
    fn test_opcode_greater() {
        let mut test_vm = VM::new();
        test_vm.program = vec![1, 20, 12, 34, 1, 22, 43, 21, 11, 20, 22];
        test_vm.run();
        assert_eq!(test_vm.comparison_flag, false);
        test_vm.program.extend([1, 22, 12, 34, 11, 20, 22]);
        test_vm.run();
        assert_eq!(test_vm.comparison_flag, false);
        test_vm.program.extend([1, 22, 0, 12, 11, 20, 22]);
        test_vm.run();
        assert_eq!(test_vm.comparison_flag, true);
    }

    #[test]
    fn test_opcode_less() {
        let mut test_vm = VM::new();
        test_vm.program = vec![1, 20, 12, 34, 1, 22, 43, 21, 12, 20, 22];
        test_vm.run();
        assert_eq!(test_vm.comparison_flag, true);
        test_vm.program.extend([1, 22, 12, 34, 12, 20, 22]);
        test_vm.run();
        assert_eq!(test_vm.comparison_flag, false);
        test_vm.program.extend([1, 22, 0, 12, 12, 20, 22]);
        test_vm.run();
        assert_eq!(test_vm.comparison_flag, false);
    }

    #[test]
    fn test_opcode_greater_equal() {
        let mut test_vm = VM::new();
        test_vm.program = vec![1, 20, 12, 34, 1, 22, 43, 21, 13, 20, 22];
        test_vm.run();
        assert_eq!(test_vm.comparison_flag, false);
        test_vm.program.extend([1, 22, 12, 34, 13, 20, 22]);
        test_vm.run();
        assert_eq!(test_vm.comparison_flag, true);
        test_vm.program.extend([1, 22, 0, 12, 13, 20, 22]);
        test_vm.run();
        assert_eq!(test_vm.comparison_flag, true);
    }

    #[test]
    fn test_opcode_less_equal() {
        let mut test_vm = VM::new();
        test_vm.program = vec![1, 20, 12, 34, 1, 22, 43, 21, 14, 20, 22];
        test_vm.run();
        assert_eq!(test_vm.comparison_flag, true);
        test_vm.program.extend([1, 22, 12, 34, 14, 20, 22]);
        test_vm.run();
        assert_eq!(test_vm.comparison_flag, true);
        test_vm.program.extend([1, 22, 0, 12, 14, 20, 22]);
        test_vm.run();
        assert_eq!(test_vm.comparison_flag, false);
    }

    #[test]
    fn test_opcode_jump_if() {
        let mut test_vm = VM::new();
        test_vm.registers[5] = 15;
        test_vm.program = vec![1, 31, 0, 41, 1, 15, 0, 26, 10, 31, 15, 15, 5, 0, 0, 0];
        //                     *--==--=====  *--==--=====  **--==--==  **--=        o->
        test_vm.run();
        assert_eq!(test_vm.program_counter, 16);
    }

    #[test]
    fn test_opcode_illegal() {
        let mut test_vm = VM::new();
        test_vm.program = vec![123, 0, 0, 0];
        test_vm.run();
        assert_eq!(test_vm.program_counter, 1);
    }
}
