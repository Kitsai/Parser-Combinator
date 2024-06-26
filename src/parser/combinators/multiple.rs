use std::ops::RangeBounds;
use std::ops::Bound::{self,*};
use std::usize;

use crate::parser::Parser;

use super::pair::pair;

pub fn one_or_more<'a, O: 'a + Clone>(parser: Parser<'a, O>) -> Parser<'a, Vec<O>>
{
    pair(parser.clone(), zero_or_more(parser)).map( | (head, mut tail) | {
        tail.insert(0, head);
        tail
    })
}

pub fn zero_or_more<'a, O: 'a>(parser: Parser<'a, O>) -> Parser<'a, Vec<O>>
{
    Parser::new(
        move | mut input | {
            let mut result = Vec::new();

            while let Ok((next_input, next_item)) = parser.parse(input) {
                input = next_input;
                result.push(next_item);
            }

            Ok((input, result))
        }
    )
}

pub fn n_or_more<'a, O>(parser: Parser<'a, O>, n: usize) -> Parser<'a, Vec<O>>
where 
    O: 'a,
{
    Parser::new(
        move | input: &str | {
            let mut result: Vec<O> = Vec::new();
            let mut inp: &str = input;

            for _ in 0..n {
                if let Ok((next_input, next_item)) = parser.parse(inp) {
                    inp = next_input;
                    result.push(next_item);
                } else {
                    return Err(input);
                }
            }

            while let Ok((next_input,next_item)) = parser.parse(inp) {
                inp = next_input;
                result.push(next_item);
            }
            Ok((inp, result))
        }
    )
}

pub fn n_times<'a, O>(parser: Parser<'a, O>, n: usize) -> Parser<'a, Vec<O>>
where 
    O: 'a,
{
    Parser::new(
        move | input: &str | {
            let mut inp: &str = input;
            let mut result: Vec<O> = Vec::new();

            if n == 0 {
                return Err(input);
            }

            for _ in 0..n {
                if let Ok((next_input, next_item)) = parser.parse(inp) {
                    inp = next_input;
                    result.push(next_item);
                } else {
                    return Err(input);
                }
            }

            return Ok((inp, result));
        }
    )
}

pub fn n_to_m<'a,O>(parser: Parser<'a, O>, n: usize, m: usize) -> Parser<'a, Vec<O>>
where
    O: 'a,
{
    Parser::new(
        move | input | {
            let mut result: Vec<O> = Vec::new();
            let mut inp: &str = input;

            let mut count: usize = 0;
            loop {
                if count > m {
                    break;
                }
                if let Ok((next_input, next_item)) = parser.parse(inp) {
                    inp = next_input;
                    result.push(next_item);
                } else {
                    if count >= n {
                        break;
                    }
                    return Err(input)
                }

                count += 1;
            }

            Ok((inp, result))
        }
    )
}

pub fn repeat<'a, O, R>(parser: Parser<'a, O>, range: R) -> Parser<'a, Vec<O>>
where 
    R: RangeBounds<usize> + 'a,
    O: 'a,
{
    let start: Bound<&usize> = range.start_bound();

    let end: Bound<&usize> = range .end_bound();

    match (start, end) {
        (Unbounded, Unbounded) => zero_or_more(parser),
        (Unbounded, Included(n)) 
        | (Unbounded, Excluded(n)) => n_times(parser, *n),
        (Included(n1), Unbounded)
        | (Excluded(n1), Unbounded)
        => n_or_more(parser, *n1),
        (Included(n1), Included(n2)) if n1 == n2 => n_times(parser, *n1),
        (Included(n1), Excluded(n2)) 
        | (Included(n1),Included(n2))
        | (Excluded(n1), Included(n2))
        | (Excluded(n1), Excluded(n2))
        => n_to_m(parser, *n1, *n2),
    }
}