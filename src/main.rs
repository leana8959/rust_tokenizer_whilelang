use std::fmt;

macro_rules! token_kind {
    ( $( $kind:ident, $syntax:literal ),+ ) => {
        #[derive(Debug, PartialEq, Eq)]
        enum TokenKind { $( $kind, )+ }


        trait Token { fn starts_with_token(&self) -> Option<TokenKind>; }
        impl Token for &str {
            fn starts_with_token(&self) -> Option<TokenKind> {
                match *self {
                    $( t if t.starts_with($syntax) => Some(TokenKind::$kind), )+
                    _ => None,
                }
            }
        }

        impl fmt::Display for TokenKind {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                use TokenKind::*;
                match self {
                    $( $kind => write!(f, $syntax), )+
                }
            }
        }
    };
}

#[rustfmt::skip]
token_kind!(
    Read, "read",
    Write, "write",
    While, "while",
    Do, "do",
    Od, "od",
    Percent, "%",
    Assign, ":=",
    LParen, "(",
    RParen, ")",
    Semicolon, ";"
);

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

    fn skip_whitespace(&mut self) {
        match self.peek_char() {
            Some(c) if c.is_whitespace() => {
                self.position += 1;
                self.skip_whitespace();
            }
            _ => (),
        }
    }

    fn peek_char(&self) -> Option<char> {
        self.input.chars().nth(self.position)
    }

    fn next_char(&mut self) -> Option<char> {
        let ch = self.peek_char();
        self.position += 1;

        ch
    }

    fn remaining_input(&self) -> Option<&str> {
        if self.position >= self.input.len() {
            None
        } else {
            Some(&self.input[self.position..])
        }
    }

    fn next_token(&mut self) -> Option<TokenKind> {
        match self.remaining_input()?.starts_with_token() {
            Some(t) => {
                self.position += t.to_string().len();
                Some(t)
            }
            None => {
                self.skip_whitespace();
                None
            }
        }
    }
}

#[test]
fn next_token_test_1() {
    let mut t = Tokenizer::new("read");
    assert_eq!(Some("read"), t.remaining_input());
    assert_eq!(Some(TokenKind::Read), t.next_token());
    assert_eq!(None, t.remaining_input());
    assert_eq!(None, t.next_token());
}

#[test]
fn next_token_test_2() {
    let mut t = Tokenizer::new("%");
    assert_eq!(Some("%"), t.remaining_input());
    assert_eq!(Some(TokenKind::Percent), t.next_token());
    assert_eq!(None, t.remaining_input());
    assert_eq!(None, t.next_token());
}

#[test]
fn next_token_test_3() {
    let mut t = Tokenizer::new(" ");
    assert_eq!(Some(" "), t.remaining_input());
    assert_eq!(None, t.next_token());
    assert_eq!(None, t.remaining_input());
    assert_eq!(None, t.next_token());
}

fn main() {
    println!("Hello, world!");
}
