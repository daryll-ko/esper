#[derive(Debug, PartialEq)]
pub enum Opcode {
    HALT,
    LOAD,
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,
    JUMP,
    JUMPFORWARD,
    JUMPBACKWARD,
    ILLEGAL,
}

impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
        match v {
            0 => Opcode::HALT,
            1 => Opcode::LOAD,
            2 => Opcode::ADD,
            3 => Opcode::SUBTRACT,
            4 => Opcode::MULTIPLY,
            5 => Opcode::DIVIDE,
            6 => Opcode::JUMP,
            7 => Opcode::JUMPFORWARD,
            8 => Opcode::JUMPBACKWARD,
            _ => Opcode::ILLEGAL,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode: Opcode,
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Instruction {
        Instruction { opcode }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_halt() {
        let opcode = Opcode::HALT;
        assert_eq!(opcode, Opcode::HALT);
    }

    #[test]
    fn test_create_instruction() {
        let instruction = Instruction::new(Opcode::HALT);
        assert_eq!(instruction.opcode, Opcode::HALT);
    }
}
