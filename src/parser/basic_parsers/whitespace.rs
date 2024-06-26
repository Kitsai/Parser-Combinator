use crate::parser::{combinators::multiple::{one_or_more, zero_or_more}, Parser};

use super::char::any_char;

pub fn whitespace_char<'a>() -> Parser<'a, char> {
    any_char().pred(move |r| r.is_whitespace())
}

pub fn space0<'a>() -> Parser<'a, Vec<char>> 
{
    zero_or_more(whitespace_char())
}

pub fn space1<'a>() -> Parser<'a, Vec<char>>
{
    one_or_more(whitespace_char())
}

pub fn whitespace_wrap<'a,O>(parser: Parser<'a,O>) -> Parser<'a,O> 
where 
    O: 'a,
{
    space0() >> parser << space0()
}