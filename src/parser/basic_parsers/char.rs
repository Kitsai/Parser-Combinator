use super::super::Parser;

pub fn any_char<'a>() -> Parser<'a, char> {
    Parser::new(
        |input| match input.chars().next() {
            Some(c) => Ok((&input[1..],c)),
            _ => Err(input)
        }
    )
}

pub fn char<'a>(expected: char) -> Parser<'a, char> {
    Parser::new(
        move |input| match input.chars().next() {
            Some(c) if c == expected => Ok((&input[1..], c)),
            _ => Err(input)
        }
    )
}