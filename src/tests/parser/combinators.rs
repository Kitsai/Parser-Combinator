use crate::parser::{basic_parsers::char::any_char, combinators::map::map};

#[test]
fn map_combinator() {
    fn f(input: char) -> u8 {
        match input.to_string().parse::<u8>() {
            Ok(num) => num,
            _ => 0,
        }
    }
    let p = map(any_char(), f);

    assert_eq!(
        p.parse("23"),
        Ok(("3", 2))
    );
    assert_eq!(
        p.parse("Oi!"), 
        Ok(("i!", 0))
    );
    assert_eq!(
        p.parse(""),
        Err("")
    );

    let p = any_char();

    assert_eq!(
        p.map(f).parse("23"),
        Ok(("3", 2))
    );
    assert_eq!(
        p.map(f).parse("Oi!"), 
        Ok(("i!", 0))
    );
    assert_eq!(
        p.map(f).parse(""),
        Err("")
    );
}