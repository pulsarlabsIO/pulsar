use std::fmt;

#[derive(PartialEq, Clone, Debug)]
pub enum Token {
    IDENT(String),
    INT(i64),
    FLOAT(f64),
    STRING(String),
    ILLEGAL(String),

    EOF,
    ASSIGN,
    PLUS,
    MINUS,
    MUL,
    DIV,
    BANG,
    LT,
    GT,
    EQ,
    NOTEQ,
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
