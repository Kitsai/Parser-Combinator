use crate::parser::Parser;

use super::pair::pair;

pub fn left<'a, R1: 'a + Clone, R2: 'a + Clone>(parser1: Parser<'a, R1>, parser2: Parser<'a, R2>) -> Parser<'a, R1> 
{
    pair(parser1, parser2).map(|(left, _right)| left)
}

pub fn right<'a, R1: 'a + Clone, R2: 'a + Clone>(parser1: Parser<'a, R1>, parser2: Parser<'a, R2>) -> Parser<'a, R2>
{
    pair(parser1, parser2).map(|(_left,right)| right)
}