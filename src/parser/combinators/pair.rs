use crate::parser::Parser;

pub fn pair<'a, R1, R2>(parser1: Parser<'a, R1>, parser2: Parser<'a, R2>) -> Parser<'a, (R1,R2)>
where
    'a: 'static,
    R1: 'a + Clone,
    R2: 'a + Clone,
{
    /* 
    Parser::new(
        move| input |
        parser1.parse(input).and_then(|(next_input, result1)| 
            parser2.parse(next_input)
                .map(|(last_input, result2)| (last_input, (result1, result2)))
        )
    )*/
    parser1.bind(move| result1 | parser2.map(move | result2 | (result1.clone(),result2)))
}