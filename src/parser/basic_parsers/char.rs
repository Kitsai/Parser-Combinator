use super::super::Parser;

pub fn any_char() -> Parser<'static, char> {
    Parser::new(
        |input| match input.chars().next() {
            Some(c) => Ok((&input[1..],c)),
            _ => Err(input)
        }
    )
}

pub fn char(expected: char) -> Parser<'static, char> {
    Parser::new(
        move |input| match input.chars().next() {
            Some(c) if c == expected => Ok((&input[1..], c)),
            _ => Err(input)
        }
    )
}