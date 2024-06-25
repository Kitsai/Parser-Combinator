use crate::parser::Parser;

pub fn pair<'a, R1, R2>(parser1: Parser<'a, R1>, parser2: Parser<'a, R2>) -> Parser<'a, (R1,R2)>
where
    R1: 'a + Clone,
    R2: 'a + Clone,
{
    parser1.bind(move| result1 | parser2.map(move | result2 | (result1.clone(),result2)))
}