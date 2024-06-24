use crate::parser::Parser;

pub fn pure<'a>() -> Parser<'a, &'a str> {
    Parser::new(
        |input| Ok(("",input))
    )
}