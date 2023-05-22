use std::fmt;

macro_rules! symbol_type {
    ( $( $name:ident, $syntax:literal ),+ ) => {
        #[derive(Debug, PartialEq, Eq)]
        enum SymbolType { $( $name, )+ }

        fn break_off_token_from_str<'a>(input: &'a str) -> Option<(SymbolType, &'a str)> {
            use SymbolType::*;
            $(
                if let Some(rem) = input.strip_prefix($syntax) {
                    if rem.starts_with_whitespace() {
                        return Some(($name, rem));
                    }
                }
            )+

            return None;
        }

        // Q: remove this?
        impl fmt::Display for SymbolType {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                use SymbolType::*;
                match self { $( $name => write!(f, $syntax), )+ }
            }
        }
    };
}

#[rustfmt::skip]
symbol_type!(
    Percent, "%",
    Assign, ":=",
    Semicolon, ";",

    Read, "read",
    Write, "write",
    Do, "do",
    Od, "od"
);

trait Whitespace {
    fn starts_with_whitespace(&self) -> bool;
}
impl Whitespace for &str {
    fn starts_with_whitespace(&self) -> bool {
        self.starts_with(' ') || self.starts_with('\n') || self.starts_with('\t') || self.is_empty()
    }
}

enum TokenType {
    Keyword(SymbolType),
    Ident(String),
}

#[test]
fn break_off_token_from_str_test() {
    use SymbolType::*;
    let input = "%";
    assert_eq!(Some((Percent, "")), break_off_token_from_str(input));
    let input = ";";
    assert_eq!(Some((Semicolon, "")), break_off_token_from_str(input));
    let input = ":=";
    assert_eq!(Some((Assign, "")), break_off_token_from_str(input));
}

fn main() {
    todo!()
}
