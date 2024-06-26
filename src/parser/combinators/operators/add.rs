use std::ops::Add;

use crate::parser::{combinators::pair::pair, Parser};

/// pair combinator
impl<'a, O, U> Add<Parser<'a, U>> for Parser<'a,O> 
where
    O: Clone + 'a,
    U: Clone + 'a,
{
    type Output = Parser<'a,(O,U)>;

    fn add(self, rhs: Parser<'a, U>) -> Self::Output {
        pair(self, rhs)
    }
}