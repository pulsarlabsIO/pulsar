use core::fmt;

#[derive(PartialEq, Clone, Debug)]
pub enum Token {
    Ident(String),
    Int(i64),
    Float(f64),
    String(String),
    Illegal(String),

    Eof,
    Eq,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    LessThan,
    GreaterThan,
    EqEq,
    BangEq,
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,

    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
