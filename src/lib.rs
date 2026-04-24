pub mod lox;
use std::process;
use crate::lox::Token;
use crate::lox::TokenType;


#[derive (Debug, PartialEq)]
struct ParserError {
    error_msg: String,
    error_token: Token,
}



pub fn error (line:usize, message: &str) {
    report(line, "", message);
    process::exit(1)
}

pub fn parse_error(token : Token, message: String) {
    if token.token_type == TokenType::EOF {
        report(token.line, "at end", &message)
    }
    else {
        report(token.line, format!("at '{}'", token.lexeme).as_str(), &message);
    }
}

fn report (line:usize, where_at:&str, message:&str) {
    eprintln!("[Line {line} ] Error {where_at} : {message}");
}


