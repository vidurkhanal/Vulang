use crate::assembler::Token;
use nom::{
    character::complete::{char, digit1, space0, space1},
    sequence::tuple,
    IResult,
};

pub fn register(input: &str) -> IResult<&str, Token> {
    let (remaining_input, (_, _, reg_num)) = tuple((char('$'), space0, digit1))(input)?;
    let (remaining_input, _) = space1(remaining_input)?;
    Ok((
        remaining_input,
        Token::Register {
            reg_num: reg_num.parse::<u8>().unwrap(),
        },
    ))
}

#[cfg(test)]
mod register_parser_test {
    use super::*;
    use crate::assembler::Token;

    #[test]
    fn test_register_parser() {
        let result = register("$0 123");
        assert_eq!(result.is_ok(), true);
        let (remaining, token) = result.unwrap();
        assert_eq!(remaining, "123");
        assert_eq!(token, Token::Register { reg_num: 0 });
    }
}
