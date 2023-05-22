mod token_type;

use crate::token_type::{SymbolType, TokenType};

#[derive(Default, Debug)]
struct Tokenizer {
    input: String,
    position: usize,
}

impl Tokenizer {
    fn new(input: String) -> Self {
        Tokenizer {
            input,
            ..Default::default()
        }
    }
}

impl Iterator for Tokenizer {
    type Item = TokenType;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.input[self.position..] {
            "" => None,
            input => match SymbolType::strip_token(&input) {
                Some(symbol) => {
                    self.position += symbol.len();
                    Some(TokenType::Symbol(symbol))
                }
                None => None,
            },
        }
    }
}

fn main() {
    todo!()
}
