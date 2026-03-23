use crate::lox_scanner::TokenType;
use crate::lox_scanner::Object;

struct Token<'a> {
    token_type: TokenType,
    lexeme: &'a str,
    literal:Object,
    line: u64,
}

impl Token {
    pub fn toString(&self) -> &'static str{
        "{self.type} {self.lexeme} {self.literal}"
    } 
}
