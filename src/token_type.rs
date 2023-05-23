use std::fmt;

macro_rules! symbol_type {
    ( $( $name:ident, $syntax:literal ),+ ) => {
        #[derive(Debug, PartialEq, Eq)]
        pub enum SymbolType { $( $name, )+ }

        impl SymbolType {
            pub fn len(&self) -> usize {
                use SymbolType::*;
                match self {
                    $( $name => $syntax.len(), )+
                }
            }
        }

        impl TokenType {
            pub fn strip_symbol<'a>(input: &'a str) -> Option<SymbolType> {
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

impl TokenType {
    pub fn strip_identifier(input: &str) -> Option<&str> {
        let mut cursor = 0;
        let mut rem = &input[cursor..];

        while !rem.starts_with_token() {
            cursor += 1;
            rem = &input[cursor..];
        }

        let result = &input[..cursor];
        if result == "" {
            None
        } else {
            Some(result)
        }
    }

    pub fn len(&self) -> usize {
        use TokenType::*;
        match self {
            Symbol(symbol) => symbol.len(),
            Ident(id) => id.len(),
        }
    }
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
    fn starts_with_whitespace(&self) -> bool;
    fn starts_with_token(&self) -> bool;
}
impl Tokenizable for &str {
    fn starts_with_whitespace(&self) -> bool {
        self.starts_with(' ') || self.starts_with('\n') || self.starts_with('\t')
    }
    fn starts_with_token(&self) -> bool {
        self.starts_with_whitespace() || self.is_empty() || TokenType::strip_symbol(self).is_some()
    }
}

#[test]
fn strip_identifier_test() {
    use TokenType::Ident;
    assert_eq!(Some("aoeu"), TokenType::strip_identifier("aoeu"));
}

#[test]
fn strip_symbol_test() {
    use SymbolType::*;
    let input = "%";
    assert_eq!(Some(Percent), TokenType::strip_symbol(input));
    let input = ";";
    assert_eq!(Some(Semicolon), TokenType::strip_symbol(input));
    let input = ":=";
    assert_eq!(Some(Assign), TokenType::strip_symbol(input));
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    Symbol(SymbolType),
    Ident(String),
}
