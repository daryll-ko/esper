use nom::character::complete::space0;
use nom::sequence::tuple;
use nom::IResult;

use super::opcode_parsers::opcode_load;
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

pub fn one_instruction(input: &str) -> IResult<&str, AssemblerInstruction> {
    let (input, _) = space0(input)?;
    let (input, (opcode, operand1, operand2)) =
        tuple((opcode_load, register, integer_operand))(input)?;
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
