use std::fmt;

macro_rules! symbol_type {
    ( $( $name:ident, $syntax:literal ),+ ) => {
        #[derive(Debug, PartialEq, Eq)]
        pub enum SymbolType { $( $name, )+ }

        impl SymbolType {
            pub fn strip_token<'a>(input: &'a str) -> Option<SymbolType> {
                use SymbolType::*;
                $(
                    if let Some(rem) = input.strip_prefix($syntax) {
                        if rem.starts_with_token() {
                            return Some($name);
                        }
                    }
                )+

                return None;
            }

            pub fn len(&self) -> usize {
                use SymbolType::*;
                match self {
                    $( $name => $syntax.len(), )+
                }
            }
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

trait Tokenizable {
    fn starts_with_token(&self) -> bool;
}
impl Tokenizable for &str {
    fn starts_with_token(&self) -> bool {
        self.starts_with(' ')
            || self.starts_with('\n')
            || self.starts_with('\t')
            || self.is_empty()
            || SymbolType::strip_token(self).is_some()
    }
}

#[test]
fn strip_token_test() {
    use SymbolType::*;
    let input = "%";
    assert_eq!(Some(Percent), SymbolType::strip_token(input));
    let input = ";";
    assert_eq!(Some(Semicolon), SymbolType::strip_token(input));
    let input = ":=";
    assert_eq!(Some(Assign), SymbolType::strip_token(input));
}

pub enum TokenType {
    Symbol(SymbolType),
    Ident(String),
}
