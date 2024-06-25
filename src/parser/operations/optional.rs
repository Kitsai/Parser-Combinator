use crate::parser::Parser;

pub fn make_optional<'a,O>(parser: Parser<'a,O>) -> Parser<'a,Option<O>>
where 
    O: 'a,
{
        Parser::new(
            move | input | match parser.parse(input) {
                Ok((next_input, res)) => Ok((next_input, Some(res))),
                Err(_) => Ok((input, None))
            }
        )
       
}