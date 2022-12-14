use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space0},
    sequence::tuple,
    IResult,
};

use super::Token;

pub fn register(input: &str) -> IResult<&str, Token> {
    let (input, _) = space0(input)?;
    let (input, (_, index)) = tuple((tag("$"), digit1))(input)?;
    let (input, _) = space0(input)?;
    Ok((
        input,
        Token::Register {
            index: index.parse::<u8>().unwrap(),
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_register() {
        let result = register("$0");
        assert_eq!(result, Ok(("", Token::Register { index: 0 })));

        let result = register("   $5   ");
        assert_eq!(result, Ok(("", Token::Register { index: 5 })));

        let result = register("A");
        assert_eq!(result.is_ok(), false);

        let result = register("$f");
        assert_eq!(result.is_ok(), false);
    }
}
