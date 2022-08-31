use crate::instruction::Opcode;

pub struct VM {
    registers: [i32; 32],
    program_counter: usize,
    program: Vec<u8>,
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            program_counter: 0,
            program: vec![],
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

    pub fn run(&mut self) {
        loop {
            if self.program_counter >= self.program.len() {
                break;
            }
            match self.decode_opcode() {
                Opcode::HALT => {
                    println!("HALT encountered");
                    return;
                }
                Opcode::LOAD => {
                    let register = self.next_8_bits() as usize;
                    let number = self.next_16_bits() as u16;
                    self.registers[register] = number as i32;
                    continue;
                }
                _ => {
                    println!("Unrecognized opcode found. Terminating...");
                    return;
                }
            }
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
        let test_bytes = vec![0, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.program_counter, 1);
    }

    #[test]
    fn test_load_opcode() {
        let mut test_vm = VM::new();
        test_vm.program = vec![1, 0, 1, 58, 0];
        test_vm.run();
        assert_eq!(test_vm.registers[0], 314);
    }

    #[test]
    fn test_opcode_illegal() {
        let mut test_vm = VM::new();
        let test_bytes = vec![123, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.program_counter, 1);
    }
}
