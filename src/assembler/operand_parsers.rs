use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space0},
    sequence::tuple,
    IResult,
};

use super::Token;

fn integer_operand(input: &str) -> IResult<&str, Token> {
    let (input, _) = space0(input)?;
    let (input, (_, number)) = tuple((tag("#"), digit1))(input)?;
    let (input, _) = space0(input)?;
    Ok((
        input,
        Token::IntegerOperand {
            value: number.parse::<i32>().unwrap(),
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_integer_operand() {
        let result = integer_operand("#2022");
        assert_eq!(result.is_ok(), true);
        let (rest, token) = result.unwrap();
        assert_eq!(rest, "");
        assert_eq!(token, Token::IntegerOperand { value: 2022 });

        let result = integer_operand("123");
        assert_eq!(result.is_err(), true);
    }
}
