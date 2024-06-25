use crate::parser::Parser;

pub fn skip<'a,O>(parser: Parser<'a,O>, n: usize) -> Parser<'a,O>
where 
    O: 'a,
{
    Parser::new(
        move | input | parser.parse(&input[n..])
    )
}