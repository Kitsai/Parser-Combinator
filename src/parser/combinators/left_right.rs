use crate::parser::Parser;

use super::pair::pair;

pub fn left<'a, R1, R2>(parser1: Parser<'a, R1>, parser2: Parser<'a, R2>) -> Parser<'a, R1> 
where 
    R1: 'a,
    R2: 'a,
{
    pair(parser1, parser2).map(|(left, _right)| left)
}

pub fn right<'a, R1, R2>(parser1: Parser<'a, R1>, parser2: Parser<'a, R2>) -> Parser<'a, R2>
where 
    R1: 'a,
    R2: 'a,
{
    pair(parser1, parser2).map(|(_left,right)| right)
}