use nom::{multi::many1, IResult};

use super::instruction_parsers::*;

#[derive(Debug, PartialEq)]
pub struct Program {
    instructions: Vec<AssemblerInstruction>,
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
}
