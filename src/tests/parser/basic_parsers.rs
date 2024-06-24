use crate::parser::basic_parsers::{char::{any_char, char}, literal::literal};

#[test]
fn any_char_parser() {
    let p = any_char();

    assert_eq!(Ok(("i!", 'O')), p.parse("Oi!"));
    assert_eq!(Err(""), p.parse(""));
}

#[test]
fn char_parser() {
    let p = char('O');

    assert_eq!(Ok(("i!", 'O')), p.parse("Oi!"));
    assert_eq!(Err("Tchau!"), p.parse("Tchau!"));
    assert_eq!(Err(""), p.parse(""));
}

#[test]
fn literal_parser() {
    let p = literal("Oi, ");

    assert_eq!(Ok(("meu nome é Lucas.", ())), p.parse("Oi, meu nome é Lucas."));
    assert_eq!(
        Err("Tchau gente!"),
        p.parse("Tchau gente!")
    );
}