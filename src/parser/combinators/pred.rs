use crate::parser::Parser;

pub fn pred<'a, O:'a, F>(parser: Parser<'a,O>, predicate: F) -> Parser<'a, O>
where 
    F: Fn(&O) -> bool + 'a,
{
    Parser::new(
        move | input | {
            if let Ok((next_input, value)) = parser.parse(input) {
                if predicate(&value) {
                    return Ok((next_input,value));
                }
            }
            Err(input)
        }
    )
}