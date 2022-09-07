use super::Token;
use crate::instruction::Opcode;
use nom::{bytes::complete::tag, IResult};

pub fn opcode_load(input: &str) -> IResult<&str, Token> {
    let (input, _) = tag("load")(input)?;
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
