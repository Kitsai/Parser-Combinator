use crate::parser::Parser;

pub fn identifier() -> Parser<'static, String> {
    Parser::new(
        |input| {
            let mut matched = String::new();
            let mut chars = input.chars();
            match chars.next() {
                Some(next) if next.is_alphabetic() => matched.push(next),
                _ => return Err(input),
            };

            while let Some(next) = chars.next() {
                if next.is_alphanumeric() || next== '-' {
                    matched.push(next);
                } else {
                    break;
                }
            }

            let next_index = matched.len();
            Ok((&input[next_index..], matched))
        }
    )
}