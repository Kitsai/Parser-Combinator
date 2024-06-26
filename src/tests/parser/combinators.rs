use crate::parser::{basic_parsers::{char::{any_char, char}, identifier::identifier, literal::literal}, combinators::{map::map, multiple::repeat, pair::pair}};

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

#[test]
fn bind_combinator() {

}

#[test]
fn pair_combinator() {
    let tag_opener = pair(literal("<"), identifier());
    assert_eq!(
        Ok(("/>", ((),"my-first-element".to_string()))),
        tag_opener.parse("<my-first-element/>")
    );
    assert_eq!(Err("ooops"), tag_opener.parse("ooops"));
    assert_eq!(Err("!oops"), tag_opener.parse("<!oops"));

    let tag_opener = literal("<") + identifier();
    assert_eq!(
        Ok(("/>", ((),"my-first-element".to_string()))),
        tag_opener.parse("<my-first-element/>")
    );
    assert_eq!(Err("ooops"), tag_opener.parse("ooops"));
    assert_eq!(Err("!oops"), tag_opener.parse("<!oops"));
}

#[test]
fn multiple_combinator() {
    assert_eq!(
        Ok(("", vec!['a','a','a','a','a'])),
        repeat(char('a'), ..).parse("aaaaa")
    )

    // assert_eq!(
    //     Ok(("", vec!['a','a','a','a','a'])),
    //     repeat(char('a'), 5).parse("aaaaa")
    // )
}