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
    let (input, (opcode, operand1, operand2)) = tuple((opcode, register, integer_operand))(input)?;
    let (input, _) = space0(input)?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction::Opcode;

    #[test]
    fn test_parse_one_instruction() {
        let result = one_instruction("load $1 #2");
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
}
