use std::rc::Rc;

use combinators::map::map;
type ParseResult<'a, O> = Result<(&'a str, O), &'a str>;

type ParserType<'a, O> = dyn Fn(&'a str) -> ParseResult<'a, O>;

#[derive(Clone)]
pub struct Parser<'a, O> {
    pub method: Rc<ParserType<'a, O>>
}

impl<'a, O> Parser<'a, O> {
    pub fn new<P> (parser: P) -> Self
    where 
        P: Fn(&'a str) -> Result<(&'a str, O), &'a str> + 'static,
    {
        Self {
            method: Rc::new(parser)
        }
    }

    pub fn parse(&self, input: &'a str) -> ParseResult<'a, O> {
        (self.method)(input)
    }

    pub fn map<F, NewO>(&self, map_fn: F) -> Parser<'a, NewO> 
    where
        'a: 'static, 
        Self: Clone + Sized + 'a,
        O: 'a,
        NewO: 'a,
        F: Fn(O) -> NewO + 'a,
    {
        map(self.clone(), map_fn)
    } 
}

pub mod basic_parsers;
pub mod combinators;