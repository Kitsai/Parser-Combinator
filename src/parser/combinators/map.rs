use crate::parser::Parser;

pub fn map<'a, F, A: 'a, B>(parser: Parser<'a, A>, map_fn: F) -> Parser<'a, B>
where 
    'a: 'static,
    F: Fn(A) -> B + 'static,
{
    Parser::new(
        move| input | 
            parser.parse(input)
            .map( |(next_input, result)| (next_input, map_fn(result))
            )
    )
}