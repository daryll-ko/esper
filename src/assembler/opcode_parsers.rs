use super::Token;
use crate::instruction::Opcode;
use nom::{bytes::complete::tag_no_case, character::complete::space0, IResult};

pub fn opcode_load(input: &str) -> IResult<&str, Token> {
    let (input, _) = space0(input)?;
    let (input, _) = tag_no_case("load")(input)?;
    let (input, _) = space0(input)?;
    Ok((input, Token::Op { code: Opcode::LOAD }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_opcode_load() {
        let result = opcode_load("load");
        assert_eq!(result.is_ok(), true);
        let (rest, token) = result.unwrap();
        assert_eq!(token, Token::Op { code: Opcode::LOAD });
        assert_eq!(rest, "");

        let result = opcode_load("store");
        assert_eq!(result.is_err(), true);
    }
}
