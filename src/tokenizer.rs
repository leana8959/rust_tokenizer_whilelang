use nom::{character::complete::multispace0, multi::many0, IResult};

use crate::token_type::nom_eq;
use crate::token_type::SymbolType;
use crate::token_type::TokenType;

use SymbolType::*;
use TokenType::*;

#[derive(Default, Debug)]
struct Tokenizer {
    input: String,
    position: usize,
}

fn parse_whitespace(input: &str) -> IResult<&str, &str> {
    multispace0(input)
}

impl Tokenizer {
    fn new(input: &str) -> Self {
        Tokenizer {
            input: input.to_owned(),
            ..Default::default()
        }
    }

    fn eat_whitespace(&mut self) {
        if let Ok((_rem, out)) = parse_whitespace(&self.input[self.position..]) {
            self.position += out.chars().count();
        }
    }
}

#[test]
fn eat_whitespace_test_1() {
    let mut t = Tokenizer::new(" x");
    assert_eq!(0, t.position);
    t.eat_whitespace();
    assert_eq!(1, t.position);
}

#[test]
fn eat_whitespace_test_2() {
    let mut t = Tokenizer::new("  x");
    assert_eq!(0, t.position);
    t.eat_whitespace();
    assert_eq!(2, t.position);
}

#[test]
fn eat_whitespace_test_3() {
    let mut t = Tokenizer::new("   aoeu");
    assert_eq!(0, t.position);
    t.eat_whitespace();
    assert_eq!(3, t.position);
}

impl Iterator for Tokenizer {
    type Item = TokenType;

    fn next(&mut self) -> Option<Self::Item> {
        dbg!(self.position);
        self.eat_whitespace();
        dbg!(self.position);

        match TokenType::parse_token(&self.input[self.position..]) {
            Ok((rem, token)) => {
                dbg!(&token);
                self.position = self.input.chars().count() - rem.chars().count();
                Some(token)
            }
            Err(_) => None,
        }
    }
}

#[test]
fn next_token_test_1() {
    let mut t = Tokenizer::new("% ;");
    assert_eq!(Some(Symbol(Percent)), t.next());
    assert_eq!(Some(Symbol(Semicolon)), t.next());
    assert_eq!(None, t.next());
}

#[test]
fn next_token_test_2() {
    let mut t = Tokenizer::new("read X)");
    assert_eq!(Some(Symbol(Read)), t.next());
    assert_eq!(Some(Ident("X".to_string())), t.next());
    assert_eq!(Some(Symbol(RParen)), t.next());
    assert_eq!(None, t.next());
}

#[test]
fn next_token_test_3() {
    #[rustfmt::skip]
    let mut t = Tokenizer::new(
r#"read X
%
    Y := nil ;
    while X do
        Y := (cons (hd X) Y) ;
        X := (tl X)
    od
%
write Y
"#);
    assert_eq!(Some(Symbol(Read)), t.next());
    assert_eq!(Some(Ident("X".to_string())), t.next());
    assert_eq!(Some(Symbol(Percent)), t.next());

    assert_eq!(Some(Ident("Y".to_string())), t.next());
    assert_eq!(Some(Symbol(Assign)), t.next());
    assert_eq!(Some(Symbol(Nil)), t.next());
    assert_eq!(Some(Symbol(Semicolon)), t.next());

    assert_eq!(Some(Symbol(While)), t.next());
    assert_eq!(Some(Ident("X".to_string())), t.next());
    assert_eq!(Some(Symbol(Do)), t.next());
    assert_eq!(Some(Ident("Y".to_string())), t.next());
    assert_eq!(Some(Symbol(Assign)), t.next());
    assert_eq!(Some(Symbol(LParen)), t.next());
    assert_eq!(Some(Symbol(Cons)), t.next());
    assert_eq!(Some(Symbol(LParen)), t.next());
    assert_eq!(Some(Symbol(Hd)), t.next());
    assert_eq!(Some(Ident("X".to_string())), t.next());
    assert_eq!(Some(Symbol(RParen)), t.next());
    assert_eq!(Some(Ident("Y".to_string())), t.next());
    assert_eq!(Some(Symbol(RParen)), t.next());
    assert_eq!(Some(Symbol(Semicolon)), t.next());
    assert_eq!(Some(Ident("X".to_string())), t.next());
    assert_eq!(Some(Symbol(Assign)), t.next());
    assert_eq!(Some(Symbol(LParen)), t.next());
    assert_eq!(Some(Symbol(Tl)), t.next());
    assert_eq!(Some(Ident("X".to_string())), t.next());
    assert_eq!(Some(Symbol(RParen)), t.next());
    assert_eq!(Some(Symbol(Od)), t.next());

    assert_eq!(Some(Symbol(Percent)), t.next());
    assert_eq!(Some(Symbol(Write)), t.next());
    assert_eq!(Some(Ident("Y".to_string())), t.next());
    assert_eq!(None, t.next());
}
