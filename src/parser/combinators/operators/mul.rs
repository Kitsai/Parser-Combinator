use std::ops::Mul;

use crate::parser::Parser;

impl<'a, O, U ,F> Mul<F> for Parser<'a, O>
where 
    F: Fn(O) -> Parser<'a, U> + 'a,
    O: Clone + 'a,
    U:'a,
{
    type Output = Parser<'a, U>;

    fn mul(self, other: F) -> Self::Output
    where 
        Self: Clone,
    {
        self.bind(other)   
    }
}