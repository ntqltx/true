#![allow(dead_code)]
use strum::EnumString;

#[derive(Debug, Copy, Clone, PartialEq, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum TokenType {
    // single character tokens
    LeftParen, RightParen,
	LeftBrace, RightBrace,
	LeftBracket, RightBracket,
	Comma, Dot, Colon, Semicolon,

    // one or two character tokens
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,
    Plus, PlusEqual,
    Minus, MinusEqual,
    Star, StarEqual,
    Slash, SlashEqual,

    // literals
    Identifier, Str, Number,

    // keywords
    If, Else, And, Or, For, While,
    Class, This, Super, Return,
    Let, Function, True, False, Nil,

    Print, Eof
}

impl std::fmt::Display for TokenType {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{:?}", self)
	}
}

#[derive(Debug, Clone)]
pub enum LiteralValue {
    NumberValue(f64),
    StringValue(String),
}

#[derive(Debug, Clone)]
pub struct Token {
    pub lexeme: String,
    pub literal: Option<LiteralValue>,
    pub token_type: TokenType,
    pub line_number: usize,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}", self.token_type, self.lexeme)
    }
}