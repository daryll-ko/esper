use nom::{
    bytes::complete::tag,
    character::{complete::satisfy, complete::space0, is_digit},
    sequence::tuple,
    IResult,
};

use super::Token;

fn register(input: &str) -> IResult<&str, Token> {
    let (input, _) = space0(input)?;
    let (input, (_, index)) = tuple((tag("$"), satisfy(|c| is_digit(c as u8))))(input)?;
    let (input, _) = space0(input)?;
    Ok((input, Token::Register { index: index as u8 }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_register() {
        let result = register("$0");
        println!("{:?}", result);
        assert_eq!(result.is_ok(), true);
        let result = register("   $5   ");
        assert_eq!(result.is_ok(), true);
        let result = register("A");
        assert_eq!(result.is_ok(), false);
        let result = register("$f");
        assert_eq!(result.is_ok(), false);
    }
}
