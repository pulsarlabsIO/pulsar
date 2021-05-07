use crate::token::Token;

pub struct Lexer {
    position: usize,
    ch: char,
    input: String,
}

impl Lexer {
    pub fn next_token(&mut self) -> Token {
        while self.ch.is_whitespace() {
            self.read_char();
        }

        let token = match self.ch {
            '\0' => Token::EOF,
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::EQ
                } else {
                    Token::ASSIGN
                }
            }
            '+' => Token::PLUS,
            '-' => Token::MINUS,
            '*' => Token::MUL,
            '/' => Token::DIV,
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::NOTEQ
                } else {
                    Token::BANG
                }
            }
            '<' => Token::LT,
            '>' => Token::GT,
            ',' => Token::COMMA,
            ';' => Token::SEMICOLON,
            '(' => Token::LPAREN,
            ')' => Token::RPAREN,
            '{' => Token::LBRACE,
            '}' => Token::RBRACE,
            '\'' | '"' | '`' => self.identify_string(self.ch),
            _ => return self.identify_token(),
        };
        self.read_char();
        return token;
    }

    fn identify_token(&mut self) -> Token {
        let pos = self.position;
        while self.ch.is_alphanumeric() {
            self.read_char();
        }

        let literal = String::from(&self.input[pos..self.position]);
        if literal.len() == 0 {
            let ch = self.ch;
            self.read_char();
            return Token::ILLEGAL(ch.to_string());
        }

        if literal.chars().all(char::is_numeric) {
            if self.ch == '.' && self.peek_char().is_numeric() {
                self.read_char();
                while self.ch.is_numeric() {
                    self.read_char();
                }
                let literal = String::from(&self.input[pos..self.position]);
                match literal.parse::<f64>() {
                    Ok(f) => return Token::FLOAT(f),
                    Err(e) => println!("Error parsing FLOAT {}: {}", literal, e),
                }
            } else {
                match literal.parse::<i64>() {
                    Ok(i) => return Token::INT(i),
                    Err(e) => println!("Error parsing INT {}: {}", literal, e),
                }
            }
        }

        match literal.as_str() {
            "fn" => Token::FUNCTION,
            "let" => Token::LET,
            "true" => Token::TRUE,
            "false" => Token::FALSE,
            "if" => Token::IF,
            "else" => Token::ELSE,
            "return" => Token::RETURN,
            _ => Token::IDENT(literal),
        }
    }

    fn identify_string(&mut self, quote: char) -> Token {
        self.read_char();
        let pos = self.position;
        while self.ch != quote {
            self.read_char();
        }
        Token::STRING(String::from(&self.input[pos..self.position]))
    }

    fn read_char(&mut self) {
        let ch = self.peek_char();
        self.ch = ch;
        self.position += 1;
    }

    fn peek_char(&self) -> char {
        let pos = self.position + 1;
        self.read_char_at(pos)
    }

    fn read_char_at(&self, pos: usize) -> char {
        match self.input.chars().nth(pos) {
            Some(ch) => ch,
            None => '\0',
        }
    }

    pub fn new_lexer(input: String) -> Lexer {
        let mut lexer = Lexer {
            position: 0,
            ch: '\0',
            input,
        };
        lexer.ch = lexer.read_char_at(0);
        return lexer;
    }
}
