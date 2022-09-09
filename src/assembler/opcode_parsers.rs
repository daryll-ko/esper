use super::Token;
use crate::instruction::Opcode;
use nom::{
    character::complete::{alpha1, space0},
    IResult,
};

pub fn opcode(input: &str) -> IResult<&str, Token> {
    let (input, _) = space0(input)?;
    let (input, opcode) = alpha1(input)?;
    let (input, _) = space0(input)?;
    Ok((
        input,
        Token::Op {
            code: Opcode::from(opcode),
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_opcode() {
        let result = opcode("load");
        assert_eq!(result.is_ok(), true);
        let (rest, token) = result.unwrap();
        assert_eq!(token, Token::Op { code: Opcode::LOAD });
        assert_eq!(rest, "");

        let result = opcode("Toad");
        let (_, token) = result.unwrap();
        assert_eq!(
            token,
            Token::Op {
                code: Opcode::ILLEGAL
            }
        )
    }
}
