use nom::{
    character::complete::{char, digit1, space0, space1},
    sequence::tuple,
    IResult,
};

use crate::assembler::Token;

fn integer_operand(input: &str) -> IResult<&str, Token> {
    let (remaining_input, (_, _, reg_num)) = tuple((char('#'), space0, digit1))(input)?;
    let (remaining_input, _) = space1(remaining_input)?;
    Ok((
        remaining_input,
        Token::IntegerOperand {
            value: reg_num.parse::<i32>().unwrap(),
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_operand_positive() {
        assert_eq!(
            integer_operand("#1234   "),
            Ok(("", Token::IntegerOperand { value: 1234 },))
        );
        assert_eq!(
            integer_operand("#0 "),
            Ok(("", Token::IntegerOperand { value: 0 },))
        );
    }

    #[test]
    fn test_integer_operand_negative() {
        assert!(integer_operand(" #1234").is_err());
        assert!(integer_operand("#12a4").is_err());
        assert!(integer_operand("#12345678901234567890").is_err());
    }
}
