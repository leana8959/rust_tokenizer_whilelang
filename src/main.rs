use std::fmt;

macro_rules! token_kind {
    ( $( $kind:ident, $syntax:literal ),+ ) => {
        #[derive(Debug, PartialEq, Eq)]
        enum TokenKind {
            Ident(String),
            $( $kind, )+
        }

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
                    Ident(s) => write!(f, "{s}"),
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
    Nil, "nil",
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
        while let Some(c) = self.peek_char() {
            if c.is_ascii_whitespace() {
                self.position += 1;
            } else {
                break;
            }
        }
    }

    fn peek_char(&self) -> Option<char> {
        self.input.chars().nth(self.position)
    }

    fn remaining_input(&self) -> Option<&str> {
        if self.position >= self.input.len() {
            None
        } else {
            Some(&self.input[self.position..])
        }
    }

    fn next_ident(&mut self) -> Option<&str> {
        let end_position = self
            .remaining_input()?
            .find(|c: char| c.is_ascii_whitespace())
            .map(|p| p + self.position)
            .unwrap_or(self.input.len());

        // TODO: simplification
        if self.position < end_position {
            let s = &self.input[self.position..end_position];
            Some(s)
        } else {
            None
        }
    }

    pub fn next_token(&mut self) -> Option<TokenKind> {
        self.skip_whitespace();

        let token = self
            .remaining_input()?
            .starts_with_token()
            .or_else(|| self.next_ident().map(|s| TokenKind::Ident(s.to_string())));

        match token {
            Some(ref t) => self.position += t.to_string().len(),
            None => self.skip_whitespace(),
        }

        token
    }
}

#[test]
fn next_ident_test_1() {
    let mut t = Tokenizer::new("foo");
    assert_eq!(Some("foo"), t.remaining_input());
    assert_eq!(Some("foo"), t.next_ident());
}

#[test]
fn next_token_test_1() {
    let mut t = Tokenizer::new("read");
    assert_eq!(Some("read"), t.remaining_input());
    assert_eq!(Some(TokenKind::Read), t.next_token());
    assert_eq!(None, t.next_token());
}

#[test]
fn next_token_test_2() {
    let mut t = Tokenizer::new("%");
    assert_eq!(Some("%"), t.remaining_input());
    assert_eq!(Some(TokenKind::Percent), t.next_token());
    assert_eq!(None, t.next_token());
}

#[test]
fn next_token_test_3() {
    let mut t = Tokenizer::new(" ");
    assert_eq!(Some(" "), t.remaining_input());
    assert_eq!(None, t.next_token());
}

#[test]
fn next_token_test_4() {
    let mut t = Tokenizer::new("read X\n");
    assert_eq!(Some(TokenKind::Read), t.next_token());
    assert_eq!(Some(TokenKind::Ident("X".to_string())), t.next_token());
    assert_eq!(None, t.next_token());
}

#[test]
fn next_token_test_5() {
    #[rustfmt::skip]
    let mut t = Tokenizer::new(
r#"read X
%
%
write Y
"#,
    );
    assert_eq!(Some(TokenKind::Read), t.next_token());
    assert_eq!(Some(TokenKind::Ident("X".to_string())), t.next_token());
    assert_eq!(Some(TokenKind::Percent), t.next_token());
    assert_eq!(Some(TokenKind::Percent), t.next_token());
    assert_eq!(Some(TokenKind::Write), t.next_token());
    assert_eq!(Some(TokenKind::Ident("Y".to_string())), t.next_token());
    assert_eq!(None, t.next_token());
}

#[test]
fn next_token_test_6() {
    use TokenKind::*;

    #[rustfmt::skip]
    let mut t = Tokenizer::new(
r#"read X
%
    while X do
        Y := nil ;
    od
%
write Y
"#,
    );

    assert_eq!(Some(Read), t.next_token());
    assert_eq!(Some(Ident("X".to_string())), t.next_token());
    assert_eq!(Some(Percent), t.next_token());

    assert_eq!(Some(While), t.next_token());
    assert_eq!(Some(Ident("X".to_string())), t.next_token());
    assert_eq!(Some(Do), t.next_token());

    assert_eq!(Some(Ident("Y".to_string())), t.next_token());
    assert_eq!(Some(Assign), t.next_token());
    assert_eq!(Some(Nil), t.next_token());
    assert_eq!(Some(Semicolon), t.next_token());

    assert_eq!(Some(Od), t.next_token());

    assert_eq!(Some(Percent), t.next_token());
    assert_eq!(Some(Write), t.next_token());
    assert_eq!(Some(Ident("Y".to_string())), t.next_token());
    assert_eq!(None, t.next_token());
}

fn main() {}
