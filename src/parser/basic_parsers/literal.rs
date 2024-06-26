use crate::parser::Parser;

pub fn literal<'a>(expected: &'a str) -> Parser<'a, ()> {
    Parser::new(
        move | input | if input.starts_with(expected) {
            Ok((&input[expected.len()..], ()))
        } else {
            Err(input)
        }
    )
}