use nom::branch::alt;
use nom::character::complete::space0;
use nom::sequence::tuple;
use nom::IResult;

use super::opcode_parsers::opcode;
use super::operand_parsers::integer_operand;
use super::register_parsers::register;
use super::Token;

#[derive(Debug, PartialEq)]
pub struct AssemblerInstruction {
    pub opcode: Token,
    pub operand1: Option<Token>,
    pub operand2: Option<Token>,
    pub operand3: Option<Token>,
}

impl AssemblerInstruction {
    fn extract_operand(t: &Token, results: &mut Vec<u8>) {
        match t {
            Token::Register { index } => {
                results.push(*index);
            }
            Token::IntegerOperand { value } => {
                let converted = *value as u16;
                let byte1 = converted as u8;
                let byte2 = (converted >> 8) as u8;
                results.push(byte2 as u8); // Big-endian
                results.push(byte1 as u8);
            }
            _ => {
                println!("Opcode found in operand field!");
                std::process::exit(1);
            }
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut results = vec![];
        match self.opcode {
            Token::Op { code } => {
                results.push(code as u8);
            }
            _ => {
                println!("Non-opcode found in opcode field!");
                std::process::exit(1);
            }
        }

        for token in [&self.operand1, &self.operand2, &self.operand3]
            .iter()
            .copied()
            .flatten()
        {
            AssemblerInstruction::extract_operand(token, &mut results);
        }
        results
    }
}

pub fn one_instruction(input: &str) -> IResult<&str, AssemblerInstruction> {
    let (input, _) = space0(input)?;
    let (input, result) = alt((
        instruction_type_one,
        instruction_type_two,
        instruction_type_three,
    ))(input)?;
    let (input, _) = space0(input)?;
    Ok((input, result))
}

// <opcode> <register> <operand> (例えLOAD $12 #34)
fn instruction_type_one(input: &str) -> IResult<&str, AssemblerInstruction> {
    let (input, (opcode, operand1, operand2)) = tuple((opcode, register, integer_operand))(input)?;
    Ok((
        input,
        AssemblerInstruction {
            opcode,
            operand1: Some(operand1),
            operand2: Some(operand2),
            operand3: None,
        },
    ))
}

// <opcode> (例えHALT)
fn instruction_type_two(input: &str) -> IResult<&str, AssemblerInstruction> {
    let (input, opcode) = opcode(input)?;
    Ok((
        input,
        AssemblerInstruction {
            opcode,
            operand1: None,
            operand2: None,
            operand3: None,
        },
    ))
}

// <opcode> <register> <register> <register> (例えADD $12 $13 $14)
fn instruction_type_three(input: &str) -> IResult<&str, AssemblerInstruction> {
    let (input, (opcode, operand1, operand2, operand3)) =
        tuple((opcode, register, register, register))(input)?;
    Ok((
        input,
        AssemblerInstruction {
            opcode,
            operand1: Some(operand1),
            operand2: Some(operand2),
            operand3: Some(operand3),
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction::Opcode;

    #[test]
    fn test_parse_type_one_instruction() {
        let result = instruction_type_one("load $1 #2");
        assert_eq!(
            result,
            Ok((
                "",
                AssemblerInstruction {
                    opcode: Token::Op { code: Opcode::LOAD },
                    operand1: Some(Token::Register { index: 1 }),
                    operand2: Some(Token::IntegerOperand { value: 2 }),
                    operand3: None,
                }
            ))
        )
    }

    #[test]
    fn test_parse_type_two_instruction() {
        let result = instruction_type_two("halt");
        assert_eq!(
            result,
            Ok((
                "",
                AssemblerInstruction {
                    opcode: Token::Op { code: Opcode::HALT },
                    operand1: None,
                    operand2: None,
                    operand3: None,
                }
            ))
        )
    }

    #[test]
    fn test_parse_type_three_instruction() {
        let result = instruction_type_three("add $12 $13 $14");
        assert_eq!(
            result,
            Ok((
                "",
                AssemblerInstruction {
                    opcode: Token::Op { code: Opcode::ADD },
                    operand1: Some(Token::Register { index: 12 }),
                    operand2: Some(Token::Register { index: 13 }),
                    operand3: Some(Token::Register { index: 14 }),
                }
            ))
        )
    }
}
