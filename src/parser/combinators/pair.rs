use crate::parser::Parser;

pub fn pair<'a, R1, R2>(parser1: Parser<R1>, parser2: Parser<R2>) -> Parser<'a, (R1,R2)>
where
    R1: 'a + Clone,
    R2: 'a,
{
    Parser::new(
        move| input |
        parser1.parse(input).and_then(
            |(next_input, result1)| 0

        )
    )
}