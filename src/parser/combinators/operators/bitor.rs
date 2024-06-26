use std::ops::BitOr;

use crate::parser::{combinators::either::either, Parser};

///either combinator
impl<'a, O> BitOr<Parser<'a,O>> for Parser<'a,O> 
where
    O: 'a,
{
    type Output = Parser<'a, O>;

    fn bitor(self, rhs: Parser<'a,O>) -> Self::Output {
        either(self, rhs)
    }
}