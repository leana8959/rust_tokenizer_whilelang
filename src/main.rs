mod token_type;

use crate::token_type::SymbolType;
use crate::token_type::TokenType;
use crate::token_type::Tokenizable;

#[derive(Default, Debug)]
struct Tokenizer {
    input: String,
    position: usize,
}

impl Tokenizer {
    fn new(input: &str) -> Self {
        Tokenizer {
            input: input.to_owned(),
            ..Default::default()
        }
    }

    fn eat_whitespace(&mut self) {
        while self.input[self.position..].starts_with_whitespace() {
            self.position += 1;
        }
    }
}

impl Iterator for Tokenizer {
    type Item = TokenType;

    fn next(&mut self) -> Option<Self::Item> {
        self.eat_whitespace();

        match &self.input[self.position..] {
            "" => None,
            input => match TokenType::strip_symbol(&input)
                .or_else(|| TokenType::strip_identifier(&input))
            {
                Some(token) => {
                    self.position += token.len();
                    Some(token)
                }
                None => None,
            },
        }
    }
}

#[test]
fn next_token_test_1() {
    use SymbolType::*;
    use TokenType::*;
    let mut t = Tokenizer::new("%;");
    assert_eq!(Some(Symbol(Percent)), t.next());
    assert_eq!(Some(Symbol(Semicolon)), t.next());
    assert_eq!(None, t.next());
}

#[test]
fn next_token_test_2() {
    use SymbolType::*;
    use TokenType::*;
    let mut t = Tokenizer::new("read   X)");
    assert_eq!(Some(Symbol(Read)), t.next());
    assert_eq!(Some(Ident("X".to_string())), t.next());
    assert_eq!(Some(Symbol(RParen)), t.next());
    assert_eq!(None, t.next());
}

#[test]
fn next_token_test_3() {
    use SymbolType::*;
    use TokenType::*;
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

fn main() {
    todo!()
}
