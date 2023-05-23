mod token_type;

use crate::token_type::{SymbolType, TokenType};

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
}

impl Iterator for Tokenizer {
    type Item = TokenType;

    fn next(&mut self) -> Option<Self::Item> {
        use TokenType::*;
        match &self.input[self.position..] {
            "" => None,
            input => match TokenType::strip_symbol(&input)
                .and_then(|symbol| Some(Symbol(symbol)))
                .or_else(|| {
                    TokenType::strip_identifier(&input)
                        .and_then(|ident| Some(Ident(ident.to_string())))
                }) {
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
    use TokenType::*;
    let mut t = Tokenizer::new("X");
    assert_eq!(Some(Ident("X".to_string())), t.next());
    assert_eq!(None, t.next());
}

fn main() {
    todo!()
}
