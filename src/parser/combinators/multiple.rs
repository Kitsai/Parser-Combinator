use std::ops::RangeBounds;
use std::ops::Bound::*;
use std::usize;

use crate::parser::Parser;

use super::pair::pair;

pub fn one_or_more<'a, O: 'a + Clone>(parser: Parser<'a, O>) -> Parser<'a, Vec<O>>
where 
    'a:'static,
{
    pair(parser.clone(), zero_or_more(parser)).map( | (head, mut tail) | {
        tail.insert(0, head);
        tail
    })
}

pub fn zero_or_more<'a, O: 'a>(parser: Parser<'a, O>) -> Parser<'a, Vec<O>>
where 
    'a: 'static,
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

// fn n_or_more<'a, O>(parser: Parser<'a, O>, n: usize) -> Parser<'a, Vec<O>>
// where 
//     'a: 'static,
// {
    
// }

fn n_times<'a, O>(parser: Parser<'a, O>, n: usize) -> Parser<'a, Vec<O>>
where 
    'a: 'static,
    O: 'a,
{
    Parser::new(
        move | mut input | {
            let mut result = Vec::new();

            if n == 0 {
                return Err(input);
            }

            if let Ok((next_input, item)) = parser.parse(input) {
                input = next_input;
                result.push(item);
            } else {
                return Err(input);
            }

            for _ in 0..n {
                if let Ok((next_input, next_item)) = parser.parse(input) {
                    input = next_input;
                    result.push(next_item);
                } else {
                    break;
                }
            }

            return Ok((input, result));
        }
    )
}

// pub fn repeat<'a, O, R>(parser: Parser<'a, O>, range: R) -> Parser<'a, Vec<O>>
// where 
//     'a: 'static,
//     R: RangeBounds<usize>,
// {
//     Parser::new(
//         move | input | {
//             let start = range.start_bound();

//             let end = range .end_bound();

//             match (start, end) {
//                 (Unbounded, Unbounded) => zero_or_more(parser).parse(input),
//                 (Included(n1), Unbounded) => {
                    
//                 }
//             }
//         }
//     )
// }