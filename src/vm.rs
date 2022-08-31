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
    fn test_opcode_illegal() {
        let mut test_vm = VM::new();
        let test_bytes = vec![123, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.program_counter, 1);
    }
}
