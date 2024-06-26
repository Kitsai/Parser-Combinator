use std::ops::{Shl, Shr};

use crate::parser::{combinators::left_right::{left, right}, Parser};

///left combinator
impl<'a, O, U> Shl<Parser<'a,U>> for Parser<'a,O>
where
    O: 'a,
    U: 'a,
{
    type Output = Parser<'a,O>; 

    fn shl(self, other: Parser<'a, U>) -> Self::Output {
        left(self, other)
    }
}


///right combinator
impl<'a, O, U> Shr<Parser<'a,U>> for Parser<'a,O> 
where 
    O: 'a,
    U: 'a,
{
    type Output = Parser<'a,U>;

    fn shr(self, other: Parser<'a, U>) -> Self::Output {
        right(self, other)   
    }
}