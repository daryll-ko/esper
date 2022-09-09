#[derive(Clone, Copy, Debug, PartialEq)]
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
    EQUAL,
    NOTEQUAL,
    GREATER,
    LESS,
    GREATEREQUAL,
    LESSEQUAL,
    JUMPIF,
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
            9 => Opcode::EQUAL,
            10 => Opcode::NOTEQUAL,
            11 => Opcode::GREATER,
            12 => Opcode::LESS,
            13 => Opcode::GREATEREQUAL,
            14 => Opcode::LESSEQUAL,
            15 => Opcode::JUMPIF,
            _ => Opcode::ILLEGAL,
        }
    }
}

impl From<&str> for Opcode {
    fn from(v: &str) -> Self {
        match v {
            "halt" => Opcode::HALT,
            "load" => Opcode::LOAD,
            "add" => Opcode::ADD,
            "subtract" => Opcode::SUBTRACT,
            "multiply" => Opcode::MULTIPLY,
            "divide" => Opcode::DIVIDE,
            "jump" => Opcode::JUMP,
            "jumpforward" => Opcode::JUMPFORWARD,
            "jumpbackward" => Opcode::JUMPBACKWARD,
            "equal" => Opcode::EQUAL,
            "notequal" => Opcode::NOTEQUAL,
            "greater" => Opcode::GREATER,
            "less" => Opcode::LESS,
            "greaterequal" => Opcode::GREATEREQUAL,
            "lessequal" => Opcode::LESSEQUAL,
            "jumpif" => Opcode::JUMPIF,
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
