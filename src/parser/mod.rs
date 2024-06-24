use std::rc::Rc;
type ParseResult<'a, O> = Result<(&'a str, O), &'a str>;

type ParserType<'a, O> = dyn Fn(&'a str) -> ParseResult<'a, O>;

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
}