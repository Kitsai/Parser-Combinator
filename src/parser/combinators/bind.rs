use crate::parser::Parser;

pub fn bind<'a, F, A: 'a, B>(parser: Parser<'a, A>, f: F) -> Parser<'a, B>
where
    'a: 'static,
    F: Fn(A) -> Parser<'a, B> + 'a,
{
    Parser::new(
        move | input | match parser.parse(input) {
            Ok((next_input, result)) => f(result).parse(next_input),
            Err(err) => Err(err)
        }
    )
}