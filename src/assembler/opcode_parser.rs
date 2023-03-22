use nom::bytes::streaming::tag;
use nom::character::complete::space1;
use nom::{IResult, Parser};

use crate::assembler::Token;
use crate::OpCode;
// use nom::character::complete;

// named!(opcode_load<CompleteStr,Token>,do_parse!(tag!("load") >> (Token::Op { code: OpCode::LOAD })));

pub fn opcode_load(input: &str) -> IResult<&str, Token> {
    let (input, _) = tag("load").parse(input)?;
    let (input, _) = space1(input)?;
    Ok((input, Token::Op { code: OpCode::LOAD }))
}

mod tests {
    use super::*;
    use crate::assembler::Token;

    #[test]
    fn test_opcode_load() {
        let result = opcode_load("load 123");
        assert_eq!(result.is_ok(), true);
        let (rest, token) = result.unwrap();
        assert_eq!(token, Token::Op { code: OpCode::LOAD });
        assert_eq!(rest, "123");

        // Tests that an invalid opcode isn't recognized
        let result = opcode_load("aold");
        assert_eq!(result.is_ok(), false);
    }
}
