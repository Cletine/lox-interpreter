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
        let sample_source = String::from("");
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
        let sample_source = String::from("+");
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
        // let sample_source = vec![Some(&')'), Some(&'('),Some(&'{'),Some(&'}'),Some(&','),Some(&'.'),Some(&'-'),Some(&'+'),Some(&';'),Some(&'*'),];
        let sample_source = String::from(")({},.-+;*"); 
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
            //let sample_source = vec![Some(&'!'), Some(&' '),Some(&'!'),Some(&'='),Some(&' '),Some(&'='),Some(&' '),Some(&'='),Some(&'='),Some(&' '), Some(&'<'), Some(&' '),Some(&'<'),Some(&'='),Some(&' '),Some(&'>'),Some(&' '),Some(&'>'),Some(&'=')];
            let sample_source = String::from("! != = == < <= > >=");
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
                //let sample_source = vec![Some(&'/'),Some(&'/'),Some(&'1'),Some(&'+'),Some(&'1'),Some(&'\n'),Some(&'='),];
                let sample_source = String::from("//1+1\n=");
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
                //let mut sample_source = vec![Some(&' '),Some(&'\t'),Some(&'\r'),Some(&'='),Some(&'\n')];
                let sample_source = String::from(" \t\r=\n");
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

        #[test]
            fn indentifier() {
                /* let sample_source = vec![Some(&'a'), Some(&'n'),Some(&'d'),Some(&' '),
                                             Some(&'o'),Some(&'r'),Some(&' '),
                                             Some(&'c'),Some(&'l'),Some(&'a'),Some(&'s'),Some(&'s'),Some(&' '),
                                             Some(&'e'), Some(&'l'),Some(&'s'),Some(&'e'),Some(&' '),
                                             Some(&'f'), Some(&'a'),Some(&'l'),Some(&'s'),Some(&'e'),Some(&' '),
                                             Some(&'f'), Some(&'o'),Some(&'r'),Some(&' '),
                                             Some(&'f'), Some(&'u'),Some(&'n'),Some(&' '),
                                             Some(&'i'), Some(&'f'),Some(&' '),
                                             Some(&'n'), Some(&'i'),Some(&'l'),Some(&' '),
                                             Some(&'p'), Some(&'r'),Some(&'i'),Some(&'n'),Some(&'t'),Some(&' '),
                                             Some(&'r'), Some(&'e'),Some(&'t'),Some(&'u'),Some(&'r'),Some(&'n'),Some(&' '),
                                             Some(&'s'), Some(&'u'),Some(&'p'),Some(&'e'),Some(&'r'),Some(&' '),
                                             Some(&'t'), Some(&'h'),Some(&'i'),Some(&'s'),Some(&' '),
                                             Some(&'t'), Some(&'r'),Some(&'u'),Some(&'e'),Some(&' '),
                                             Some(&'v'), Some(&'a'),Some(&'r'),Some(&' '),
                                             Some(&'w'), Some(&'h'),Some(&'i'),Some(&'l'),Some(&'e'),Some(&' ')];*/
                let sample_source = String::from("and or class else false for fun if nil print return super this true var while ");

                let mut scanner = LoxScanner{
                    source: sample_source, 
                    Tokens: Vec::new(),
                    start: 0,
                    current: 0,
                    line: 1,
                };

                scanner.scanTokens();
                assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::EOF, lexeme: "".to_string(), literal: Object::NULL, line: 1 }));  
                assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::WHILE, lexeme: "while".to_string(), literal: Object::NULL, line: 1 }));  
                assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::VAR, lexeme: "var".to_string(), literal: Object::NULL, line: 1 }));  
                assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::TRUE, lexeme: "true".to_string(), literal: Object::NULL, line: 1 }));  
                assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::THIS, lexeme: "this".to_string(), literal: Object::NULL, line: 1 }));  
                assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::SUPER, lexeme: "super".to_string(), literal: Object::NULL, line: 1 }));  
                assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::RETURN, lexeme: "return".to_string(), literal: Object::NULL, line: 1 }));  
                assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::PRINT, lexeme: "print".to_string(), literal: Object::NULL, line: 1 }));  
                assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::NIL, lexeme: "nil".to_string(), literal: Object::NULL, line: 1 }));  
                assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::IF, lexeme: "if".to_string(), literal: Object::NULL, line: 1 }));  
                assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::FUN, lexeme: "fun".to_string(), literal: Object::NULL, line: 1 }));  
                assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::FOR, lexeme: "for".to_string(), literal: Object::NULL, line: 1 }));  
                assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::FALSE, lexeme: "false".to_string(), literal: Object::NULL, line: 1 }));  
                assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::ELSE, lexeme: "else".to_string(), literal: Object::NULL, line: 1 }));  
                assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::CLASS, lexeme: "class".to_string(), literal: Object::NULL, line: 1 }));  
                assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::OR, lexeme: "or".to_string(), literal: Object::NULL, line: 1 }));  
                assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::AND, lexeme: "and".to_string(), literal: Object::NULL, line: 1 }));  
            }
        #[test]
            fn maximal_munch() {
                /* let mut sample_source = vec![Some(&'a'), Some(&'n'),Some(&'d'),Some(&' '),Some(&'a'),Some(&'n'),Some(&'d'),Some(&'y'),Some(&' '),
                                             Some(&'o'),Some(&'r'),Some(&' '),Some(&'o'),Some(&'r'),Some(&'c'),Some(&'a'),Some(&' '),
                                             Some(&'c'),Some(&'l'),Some(&'a'),Some(&'s'),Some(&'s'),Some(&' '),
                                             Some(&'c'), Some(&'l'),Some(&'a'),Some(&'s'),Some(&'s'),Some(&'i'),Some(&'c'),Some(&' '),
                                             Some(&'e'), Some(&'l'),Some(&'s'),Some(&'e'),Some(&' '),Some(&'e'),Some(&'l'),Some(&'s'),Some(&'e'),Some(&'a'),Some(&' '),
                                             Some(&'f'), Some(&'a'),Some(&'l'),Some(&'s'),Some(&'e'),Some(&' '),Some(&'f'),Some(&'a'),Some(&'l'),Some(&'s'),Some(&'e'),Some(&'t'),Some(&' '),
                                             Some(&'f'), Some(&'o'),Some(&'r'),Some(&' '),Some(&'f'),Some(&'o'),Some(&'r'),Some(&'e'),
                                             Some(&'f'), Some(&'u'),Some(&'n'),Some(&' '),Some(&'f'),Some(&'u'),Some(&'n'),Some(&'n'),Some(&'y'),Some(&' '),
                                             Some(&'i'), Some(&'f'),Some(&' '),Some(&'i'),Some(&'f'),Some(&'f'),Some(&'y'),Some(&' '),
                                             Some(&'n'), Some(&'i'),Some(&'l'),Some(&' '),Some(&'n'),Some(&'i'),Some(&'l'),Some(&'e'),Some(&' '),
                                             Some(&'p'), Some(&'r'),Some(&'i'),Some(&'n'),Some(&'t'),Some(&' '),Some(&'p'),Some(&'r'),Some(&'i'),Some(&'n'),Some(&'t'),Some(&'e'),Some(&'r'),Some(&' '),
                                             Some(&'r'), Some(&'e'),Some(&'t'),Some(&'u'),Some(&'r'),Some(&'n'),Some(&' '),Some(&'r'),Some(&'e'),Some(&'t'),Some(&'u'),Some(&'r'),Some(&'n'),Some(&'e'),Some(&'r'),Some(&' '),
                                             Some(&'s'), Some(&'u'),Some(&'p'),Some(&'e'),Some(&'r'),Some(&' '),Some(&'s'),Some(&'u'),Some(&'p'),Some(&'e'),Some(&'r'),Some(&'f'),Some(&'l'),Some(&' '),
                                             Some(&'t'), Some(&'h'),Some(&'i'),Some(&'s'),Some(&' '),Some(&'t'),Some(&'h'),Some(&'i'),Some(&'s'),Some(&'t'),Some(&' '),
                                             Some(&'t'), Some(&'r'),Some(&'u'),Some(&'e'),Some(&' '),Some(&'t'),Some(&'r'),Some(&'u'),Some(&'e'),Some(&'s'),Some(&'t'),Some(&' '),
                                             Some(&'v'), Some(&'a'),Some(&'r'),Some(&' '),Some(&'v'),Some(&'a'),Some(&'r'),Some(&'y'),Some(&' '),Some(&' '),
                                             Some(&'w'), Some(&'h'),Some(&'i'),Some(&'l'),Some(&'e'),Some(&' '),Some(&'w'),Some(&'h'),Some(&'i'),Some(&'l'),Some(&'e'),Some(&'r'),Some(&' ')];*/

                let sample_source = String::from("and andy or orca class  classic else elsea false falset for fore fun funny if iffy nil nile print printer return returner super superfl this thist true truest var vary while whiler");
                let mut scanner = LoxScanner{
                    source: sample_source, 
                    Tokens: Vec::new(),
                    start: 0,
                    current: 0,
                    line: 1,
                };

                scanner.scanTokens();
                assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::EOF, lexeme: "".to_string(), literal: Object::NULL, line: 1 }));  
                assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::IDENTIFIER, lexeme: "whiler".to_string(), literal: Object::NULL, line: 1 }));  
                assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::WHILE, lexeme: "while".to_string(), literal: Object::NULL, line: 1 }));  
                assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::IDENTIFIER, lexeme: "vary".to_string(), literal: Object::NULL, line: 1 }));  
                assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::VAR, lexeme: "var".to_string(), literal: Object::NULL, line: 1 }));  
                assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::IDENTIFIER, lexeme: "truest".to_string(), literal: Object::NULL, line: 1 }));  
                assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::TRUE, lexeme: "true".to_string(), literal: Object::NULL, line: 1 }));  
                assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::IDENTIFIER, lexeme: "thist".to_string(), literal: Object::NULL, line: 1 }));  
                assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::THIS, lexeme: "this".to_string(), literal: Object::NULL, line: 1 }));  
                assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::IDENTIFIER, lexeme: "superfl".to_string(), literal: Object::NULL, line: 1 }));  
                assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::SUPER, lexeme: "super".to_string(), literal: Object::NULL, line: 1 }));  
                assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::IDENTIFIER, lexeme: "returner".to_string(), literal: Object::NULL, line: 1 }));  
                assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::RETURN, lexeme: "return".to_string(), literal: Object::NULL, line: 1 }));  
                assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::IDENTIFIER, lexeme: "printer".to_string(), literal: Object::NULL, line: 1 }));  
                assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::PRINT, lexeme: "print".to_string(), literal: Object::NULL, line: 1 }));  
                assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::IDENTIFIER, lexeme: "nile".to_string(), literal: Object::NULL, line: 1 }));  
                assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::NIL, lexeme: "nil".to_string(), literal: Object::NULL, line: 1 }));  
                assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::IDENTIFIER, lexeme: "iffy".to_string(), literal: Object::NULL, line: 1 }));  
                assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::IF, lexeme: "if".to_string(), literal: Object::NULL, line: 1 }));  
                assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::IDENTIFIER, lexeme: "funny".to_string(), literal: Object::NULL, line: 1 }));  
                assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::FUN, lexeme: "fun".to_string(), literal: Object::NULL, line: 1 }));  
                assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::IDENTIFIER, lexeme: "fore".to_string(), literal: Object::NULL, line: 1 }));  
                assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::FOR, lexeme: "for".to_string(), literal: Object::NULL, line: 1 }));  
                assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::IDENTIFIER, lexeme: "falset".to_string(), literal: Object::NULL, line: 1 }));  
                assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::FALSE, lexeme: "false".to_string(), literal: Object::NULL, line: 1 }));  
                assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::IDENTIFIER, lexeme: "elsea".to_string(), literal: Object::NULL, line: 1 }));  
                assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::ELSE, lexeme: "else".to_string(), literal: Object::NULL, line: 1 }));  
                assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::IDENTIFIER, lexeme: "classic".to_string(), literal: Object::NULL, line: 1 }));  
                assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::CLASS, lexeme: "class".to_string(), literal: Object::NULL, line: 1 }));  
                assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::IDENTIFIER, lexeme: "orca".to_string(), literal: Object::NULL, line: 1 }));  
                assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::OR, lexeme: "or".to_string(), literal: Object::NULL, line: 1 }));  
                assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::IDENTIFIER, lexeme: "andy".to_string(), literal: Object::NULL, line: 1 }));  
                assert_eq!(scanner.Tokens.pop(), Some(Token { token_type: TokenType::AND, lexeme: "and".to_string(), literal: Object::NULL, line: 1 }));  
            }


}
