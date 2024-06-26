use crate::parser::Parser;

pub fn fail<'a, O>(parser: Parser<'a,O>) -> Parser<'a,()>
where
    O: 'a,
{
    Parser::new(
        move | input | {
            let inp = input;

            match parser.parse(inp) {
                Ok(_) => Ok((input, ())),
                Err(err) => Err(err)
            }
        }
    )
}