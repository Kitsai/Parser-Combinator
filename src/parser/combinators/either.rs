use crate::parser::Parser;

pub fn either<'a, O: 'a>(parser1: Parser<'a, O>, parser2: Parser<'a, O>) -> Parser<'a, O>
where 
{
    Parser::new(
        move | input | match parser1.parse(input) {
            ok @ Ok(_) => ok,
            Err(_) => parser2.parse(input)
        }
    )
} 