use std::io::stdin;

use crate::lexer::Lexer;
use crate::token::Token;

pub fn repl() {
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");
        let tokens = get_token_strings(input);
        println!("[{}]", tokens.join(", "));
    }
}

fn get_token_strings(input: String) -> Vec<String> {
    let mut lexer = Lexer::new_lexer(input);
    let mut token: Token = lexer.next_token();
    let mut tokens = Vec::new();
    while token != Token::EOF {
        tokens.push(format!("{}", token));
        token = lexer.next_token();
    }
    tokens
}
