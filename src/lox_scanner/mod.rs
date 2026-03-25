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
    fn empty_scan() {
        let mut sample_source = vec![];
        let mut scanner = LoxScanner{
            source: sample_source, 
            Tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        };
        scanner.scanTokens();
        assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::EOF, lexeme: "".to_string(), literal: Object::NULL, line: 1 }));  
    }

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

    #[test]
    fn single_char_lexeme() {
        let mut sample_source = vec![Some(&')'), Some(&'('),Some(&'{'),Some(&'}'),Some(&','),Some(&'.'),Some(&'-'),Some(&'+'),Some(&';'),Some(&'*'),];
        let mut scanner = LoxScanner{
            source: sample_source, 
            Tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        };
        scanner.scanTokens();
        assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::EOF, lexeme: "".to_string(), literal: Object::NULL, line: 1 }));  
        assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::Star, lexeme: "*".to_string(), literal: Object::NULL, line: 1 })); 
        assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::SemiColon, lexeme: ";".to_string(), literal: Object::NULL, line: 1 })); 
        assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::Plus, lexeme: "+".to_string(), literal: Object::NULL, line: 1 })); 
        assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::Minus, lexeme: "-".to_string(), literal: Object::NULL, line: 1 })); 
        assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::Dot, lexeme: ".".to_string(), literal: Object::NULL, line: 1 })); 
        assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::Comma, lexeme: ",".to_string(), literal: Object::NULL, line: 1 })); 
        assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::RightBrace, lexeme: "}".to_string(), literal: Object::NULL, line: 1 })); 
        assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::LeftBrace, lexeme: "{".to_string(), literal: Object::NULL, line: 1 })); 
        assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::LeftParen, lexeme: "(".to_string(), literal: Object::NULL, line: 1 })); 
        assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::RightParen, lexeme: ")".to_string(), literal: Object::NULL, line: 1 })); 
    }
    
    #[test]
        fn operator_lexeme() {
            let mut sample_source = vec![Some(&'!'), Some(&' '),Some(&'!'),Some(&'='),Some(&' '),Some(&'='),Some(&' '),Some(&'='),Some(&'='),Some(&' '), Some(&'<'), Some(&' '),Some(&'<'),Some(&'='),Some(&' '),Some(&'>'),Some(&' '),Some(&'>'),Some(&'=')];
            let mut scanner = LoxScanner{
                source: sample_source, 
                Tokens: Vec::new(),
                start: 0,
                current: 0,
                line: 1,
            };
            scanner.scanTokens();
            assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::EOF, lexeme: "".to_string(), literal: Object::NULL, line: 1 }));  
            assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::GreaterEqual, lexeme: ">=".to_string(), literal: Object::NULL, line: 1 }));  
            assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::Greater, lexeme: ">".to_string(), literal: Object::NULL, line: 1 }));  
            assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::LessEqual, lexeme: "<=".to_string(), literal: Object::NULL, line: 1 }));  
            assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::Less, lexeme: "<".to_string(), literal: Object::NULL, line: 1 }));  
            assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::EqualEqual, lexeme: "==".to_string(), literal: Object::NULL, line: 1 }));  
            assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::Equal, lexeme: "=".to_string(), literal: Object::NULL, line: 1 }));  
            assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::BangEqual, lexeme: "!=".to_string(), literal: Object::NULL, line: 1 }));  
            assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::Bang, lexeme: "!".to_string(), literal: Object::NULL, line: 1 }));  
        }

        #[test]
            fn comment_lexeme() {
                let mut sample_source = vec![Some(&'/'),Some(&'/'),Some(&'1'),Some(&'+'),Some(&'1'),Some(&'\n'),Some(&'='),];
                let mut scanner = LoxScanner{
                    source: sample_source, 
                    Tokens: Vec::new(),
                    start: 0,
                    current: 0,
                    line: 1,
                };
                scanner.scanTokens();
                assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::EOF, lexeme: "".to_string(), literal: Object::NULL, line: 2 }));   
                assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::Equal, lexeme: "=".to_string(), literal: Object::NULL, line: 2 }));  
            }

        #[test]
            fn empty_lexeme() {
                let mut sample_source = vec![Some(&' '),Some(&'\t'),Some(&'\r'),Some(&'='),Some(&'\n')];
                let mut scanner = LoxScanner{
                    source: sample_source, 
                    Tokens: Vec::new(),
                    start: 0,
                    current: 0,
                    line: 1,
                };
                scanner.scanTokens();
                assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::EOF, lexeme: "".to_string(), literal: Object::NULL, line: 2 }));  
                assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::Equal, lexeme: "=".to_string(), literal: Object::NULL, line: 1 }));  
            }





}
