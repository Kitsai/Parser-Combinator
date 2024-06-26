use std::rc::Rc;

use combinators::{bind::bind, map::map, pred::pred};
type ParseResult<'a, O> = Result<(&'a str, O), &'a str>;

type ParserType<'a, O> = dyn Fn(&'a str) -> ParseResult<'a, O> + 'a;

#[derive(Clone)]
pub struct Parser<'a, O> {
    pub method: Rc<ParserType<'a, O>>
}

impl<'a, O> Parser<'a, O> {
    pub fn new<P> (parser: P) -> Self
    where 
        P: Fn(&'a str) -> Result<(&'a str, O), &'a str> + 'a,
    {
        Self {
            method: Rc::new(parser)
        }
    }

    pub fn parse(&self, input: &'a str) -> ParseResult<'a, O> {
        (self.method)(input)
    }

    pub fn map<F, NewO>(self, map_fn: F) -> Parser<'a, NewO> 
    where
        Self: Sized + 'a,
        O: 'a,
        NewO: 'a,
        F: Fn(O) -> NewO + 'a,
    {
        map(self, map_fn)
    } 

    pub fn bind<F, NewO>(self, f: F) -> Parser<'a, NewO>
    where
        Self: 'a,
        O: 'a,
        NewO: 'a,
        F: Fn(O) -> Parser<'a, NewO> + 'a,
    {
        bind(self, f)
    }

    pub fn pred<F>(self, pred_fn: F) -> Parser<'a, O>
    where 
        Self: Sized + 'a,
        O: 'a,
        F: Fn(&O) -> bool + 'a,
    {
        pred(self, pred_fn)
    }
}

pub mod basic_parsers;
pub mod combinators;
pub mod operations;