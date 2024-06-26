use std::ops::Not;

use crate::parser::{operations::fail::fail, Parser};

/// not succeed.
impl<'a, O> Not for Parser<'a,O> 
where 
    O: 'a,
{
    type Output = Parser<'a,()>;

    fn not(self) -> Self::Output {
        fail(self)
    }
}