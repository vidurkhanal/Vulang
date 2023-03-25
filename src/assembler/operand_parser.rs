use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1, space0},
    combinator::opt,
    sequence::{preceded, tuple},
    IResult,
};

use crate::assembler::Token;

pub fn integer_operand(input: &str) -> IResult<&str, Token> {
    let (remaining_input, (_, _, reg_num)) = tuple((char('#'), space0, digit1))(input)?;
    let (remaining_input, _) = opt(preceded(tag("\r"), tag("\n")))(remaining_input)?;
    let (remaining_input, _) = space0(remaining_input)?;
    Ok((
        remaining_input.trim_start(),
        Token::IntegerOperand {
            value: reg_num.trim().parse::<i32>().unwrap(),
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_operand_positive() {
        assert_eq!(
            integer_operand("#1234\n   "),
            Ok(("", Token::IntegerOperand { value: 1234 },))
        );
        assert_eq!(
            integer_operand("#0\n"),
            Ok(("", Token::IntegerOperand { value: 0 },))
        );
    }
}
