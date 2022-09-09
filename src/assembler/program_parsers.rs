use super::instruction_parsers::*;
use nom::{multi::many1, IResult};

#[derive(Debug, PartialEq)]
pub struct Program {
    instructions: Vec<AssemblerInstruction>,
}

impl Program {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut program = vec![];
        for instruction in &self.instructions {
            program.append(&mut instruction.to_bytes());
        }
        program
    }
}

pub fn program(input: &str) -> IResult<&str, Program> {
    let (input, instructions) = many1(one_instruction)(input)?;
    Ok((input, Program { instructions }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assembler::Token, instruction::Opcode};

    #[test]
    fn test_parse_program() {
        let result = program("load $1 #2 load $2 #1");
        assert_eq!(
            result,
            Ok((
                "",
                Program {
                    instructions: vec![
                        AssemblerInstruction {
                            opcode: Token::Op { code: Opcode::LOAD },
                            operand1: Some(Token::Register { index: 1 }),
                            operand2: Some(Token::IntegerOperand { value: 2 }),
                            operand3: None,
                        },
                        AssemblerInstruction {
                            opcode: Token::Op { code: Opcode::LOAD },
                            operand1: Some(Token::Register { index: 2 }),
                            operand2: Some(Token::IntegerOperand { value: 1 }),
                            operand3: None,
                        }
                    ]
                }
            ))
        );
    }

    #[test]
    fn test_program_to_bytes() {
        let result = program("load $1 #2");
        assert_eq!(result.is_ok(), true);
        let (_, program) = result.unwrap();
        let bytecode = program.to_bytes();
        assert_eq!(bytecode.len(), 4);
        println!("{:?}", bytecode);
    }
}
