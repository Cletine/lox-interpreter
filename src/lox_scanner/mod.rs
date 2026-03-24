mod object;
mod scanner;
mod token;

pub use self::object::Object;
pub use self::scanner::LoxScanner;
pub use self::token::{Token, TokenType};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cons_scan() {
        let mut sample_source = vec![Some(&'+')];
        let mut scanner = LoxScanner{
            source: sample_source, 
            Tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        };
        scanner.scanTokens();
        assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::EOF, lexeme: "".to_string(), literal: Object::NULL, line: 1 }));  
        assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::Plus, lexeme: "+".to_string(), literal: Object::NULL, line: 1 })); 
    }
}
