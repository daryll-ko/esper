use crate::instruction::Opcode;

pub struct VM {
    registers: [i32; 32],
    program_counter: usize,
    program: Vec<u8>,
    remainder: u32,
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            program_counter: 0,
            program: vec![],
            remainder: 0,
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
            Opcode::ILLEGAL => true,
        }
    }

    pub fn run(&mut self) {
        let mut done = false;
        while !done {
            done = self.execute_instruction();
        }
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
    fn test_opcode_illegal() {
        let mut test_vm = VM::new();
        test_vm.program = vec![123, 0, 0, 0];
        test_vm.run();
        assert_eq!(test_vm.program_counter, 1);
    }
}
