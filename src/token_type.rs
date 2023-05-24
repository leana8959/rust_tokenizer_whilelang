use nom::{
    branch::alt,
    bytes::{complete::take_while, streaming::tag},
    combinator::map,
    IResult, Parser,
};

macro_rules! parser_func {
    ( $func_name: ident, $symbol: ident, $syntax: literal ) => {
        fn $func_name(input: &str) -> IResult<&str, TokenType> {
            use SymbolType::*;
            use TokenType::Symbol;
            map(tag($syntax), |_| Symbol($symbol))(input)
        }
    };
}

#[derive(Debug, PartialEq, Eq)]
pub enum SymbolType {
    Percent,
    Assign,
    Semicolon,
    LParen,
    RParen,
    Read,
    Write,
    While,
    Do,
    Od,
    Cons,
    Hd,
    Tl,
    Nil,
}

parser_func!(parse_percent, Percent, "%");
parser_func!(parse_assign, Assign, ":=");
parser_func!(parse_semicolon, Semicolon, ";");
parser_func!(parse_lparen, LParen, "(");
parser_func!(parse_rparen, RParen, ")");
parser_func!(parse_read, Read, "read");
parser_func!(parse_write, Write, "write");
parser_func!(parse_while, While, "while");
parser_func!(parse_do, Do, "do");
parser_func!(parse_od, Od, "od");
parser_func!(parse_cons, Cons, "cons");
parser_func!(parse_hd, Hd, "hd");
parser_func!(parse_tl, Tl, "tl");
parser_func!(parse_nil, Nil, "nil");

impl TokenType {
    pub fn parse_token(input: &str) -> IResult<&str, TokenType> {
        Self::parse_symbol(input).or(Self::parse_ident(input))
    }

    fn parse_ident(input: &str) -> IResult<&str, TokenType> {
        use TokenType::Ident;
        let is_valid_ident_name = |c: char| c.is_alphanumeric() || c == '_';

        map(take_while(is_valid_ident_name), |cs: &str| {
            Ident(cs.to_string())
        })(input)
    }

    fn parse_symbol(input: &str) -> IResult<&str, TokenType> {
        alt((
            parse_percent,
            parse_assign,
            parse_semicolon,
            parse_lparen,
            parse_rparen,
            parse_read,
            parse_write,
            parse_while,
            parse_do,
            parse_od,
            parse_cons,
            parse_hd,
            parse_tl,
            parse_nil,
        ))(input)
    }
}

#[rustfmt::skip]
macro_rules! nom_eq {
    ( $rem_expected:expr, $out_expected:expr, $parser:expr ) => {
        let (rem, out) = $parser.expect("should not fail to parse");
        assert_eq!($rem_expected, rem, "expected: {:?}\trem: {:?}", $rem_expected, rem);
        assert_eq!($out_expected, out, "expected: {:?}\tout: {:?}", $out_expected, out);
    };
}

#[test]
fn parse_ident_test_1() {
    nom_eq!(
        "",
        TokenType::Ident("aoeu".to_string()),
        TokenType::parse_ident("aoeu")
    );
}

#[test]
fn parse_ident_test_2() {
    nom_eq!(
        "",
        TokenType::Ident("should_work_for_snake_case".to_string()),
        TokenType::parse_ident("should_work_for_snake_case")
    );
}

#[test]
fn parse_ident_test_3() {
    nom_eq!(
        "",
        TokenType::Ident("shouldWorkForCamelCase".to_string()),
        TokenType::parse_ident("shouldWorkForCamelCase")
    );
}

#[test]
fn parse_symbol_test_1() {
    use SymbolType::*;
    use TokenType::Symbol;
    nom_eq!("", Symbol(Percent), TokenType::parse_symbol("%"));
    nom_eq!("", Symbol(Assign), TokenType::parse_symbol(":="));
    nom_eq!("", Symbol(Semicolon), TokenType::parse_symbol(";"));
    nom_eq!("", Symbol(LParen), TokenType::parse_symbol("("));
    nom_eq!("", Symbol(RParen), TokenType::parse_symbol(")"));
    nom_eq!("", Symbol(Read), TokenType::parse_symbol("read"));
    nom_eq!("", Symbol(Write), TokenType::parse_symbol("write"));
    nom_eq!("", Symbol(While), TokenType::parse_symbol("while"));
    nom_eq!("", Symbol(Do), TokenType::parse_symbol("do"));
    nom_eq!("", Symbol(Od), TokenType::parse_symbol("od"));
    nom_eq!("", Symbol(Cons), TokenType::parse_symbol("cons"));
    nom_eq!("", Symbol(Hd), TokenType::parse_symbol("hd"));
    nom_eq!("", Symbol(Tl), TokenType::parse_symbol("tl"));
    nom_eq!("", Symbol(Nil), TokenType::parse_symbol("nil"));
}

#[test]
fn parse_token_test_1() {
    nom_eq!(
        "",
        TokenType::Ident("shouldWorkForCamelCase".to_string()),
        TokenType::parse_token("shouldWorkForCamelCase")
    );
}

#[test]
fn parse_token_test_2() {
    nom_eq!(
        "",
        TokenType::Symbol(SymbolType::Read),
        TokenType::parse_token("read")
    );
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    Symbol(SymbolType),
    Ident(String),
}
