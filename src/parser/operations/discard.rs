use crate::parser::Parser;

pub fn discard<'a, O>(parser: Parser<'a,O>) -> Parser<'a,()>
where 
    O: Clone + 'a,
{
    parser.map(|_| ())
} 