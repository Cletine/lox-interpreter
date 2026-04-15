use crate::lox::Token;
use crate::lox::TokenType;
use crate::lox::Object;
use std::collections::HashMap;
use lazy_static::lazy_static;
use crate::error;


lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokenType> = {
        let mut m = HashMap::new();
        m.insert("and",TokenType::AND);
        m.insert("class",TokenType::CLASS);
        m.insert("else",TokenType::ELSE);
        m.insert("if",TokenType::IF);
        m.insert("nil",TokenType::NIL);
        m.insert("or",TokenType::OR);
        m.insert("print",TokenType::PRINT);
        m.insert("return",TokenType::RETURN);
        m.insert("super",TokenType::SUPER);
        m.insert("this",TokenType::THIS);
        m.insert("true",TokenType::TRUE);
        m.insert("var",TokenType::VAR);
        m.insert("fun",TokenType::FUN);
        m.insert("for",TokenType::FOR);
        m.insert("false",TokenType::FALSE);
        m.insert("while",TokenType::WHILE);
        m
    };
}

pub struct LoxScanner {
    pub source:String, 
    pub tokens: Vec<Token>,
    pub start: usize,
    pub current: usize,
    pub line: usize,
}


impl LoxScanner { 
    pub fn scan_tokens(&mut self) {
        while !(self.is_at_end()) {
            self.start = self.current;
            self.scan_token();
        }   

        self.tokens.push(
            Token{
                token_type:TokenType::EOF,
                lexeme: "".to_string(),
                literal: Object::NULL,
                line: self.line,
            });

    }

    fn scan_token(&mut self) -> () {
        let c : char = self.advance();
        match c { '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::SemiColon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                let operator = if self.match_next('=') {TokenType::BangEqual} else {TokenType::Bang};
                self.add_token(operator)
            }
            '=' => {
                let operator = if self.match_next('=') {TokenType::EqualEqual} else {TokenType::Equal};
                self.add_token(operator)
            }
            '>' => {
                let operator = if self.match_next('=') {TokenType::GreaterEqual} else {TokenType::Greater};
                self.add_token(operator) 
            }
            '<' => {
                let operator = if self.match_next('=') {TokenType::LessEqual} else {TokenType::Less};
                self.add_token(operator)
            }
            '/' => { 
                if self.match_next('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                }
                else {
                    self.add_token(TokenType::Slash);

                }
            }
            ' ' => (),
            '\r'=> (),
            '\t'=> (),
            '\n'=> self.line += 1,
            '"' => self.string(),
            _ => {
                if c.is_digit(10){
                    self.number(); 
                }
                else if c.is_alphabetic() {
                    self.identifier(); 
                }
                else {
                    error(self.current, "Unexpected Character.");
                }
            }
        }
    }

    fn advance(&mut self) -> char {
        let index = self.current;
        self.current += 1;
        self.source.as_bytes()[index] as char
    }


    fn add_token(&mut self, token_type : TokenType) -> () {
        self.add_token_literal(token_type, Object::NULL);
    }


    fn add_token_literal(&mut self, token_type:TokenType, literal: Object)-> () {
        let text : String = self.substring(self.start, self.current);
        self.tokens.push(
            Token{
                token_type:token_type,
                lexeme: text,
                literal:literal,
                line:self.line
            });
    }



    fn substring(&self, begin : usize, end : usize) -> String {
        let subslice: &str = &self.source[begin..end];
        String::from(subslice)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len() 
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        match self.source.chars().nth(self.current){
            Some(c) => { 
                if c != expected { 
                    return false;
                }
                self.current += 1;
                return true
            }
            None => return false
        } 
    }


    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        // Grab the byte at that index and cast it to a char
        self.source.as_bytes()[self.current] as char
    }

    fn peek_next(&self) -> char {
        if self.current + 1 > self.source.len() {
            return '\0';
        }
        // Grab the byte at that index and cast it to a char
        self.source.as_bytes()[self.current + 1] as char
    }


    fn string (&mut self) ->  () {
        while self.peek() != '"' && !self.is_at_end() {
            self.current +=1;
        }

        if self.is_at_end() {
            error(self.line, "Unterminated String");
        }

        self.current += 1;
        let return_str : String = self.substring(self.start + 1, self.current - 1);
        //add_tokenLiteral expects a literal of type Object
        self.add_token_literal(TokenType::STRING, Object::STRING(return_str));
    }

    fn number(&mut self) -> () {
        while self.peek().is_digit(10) {self.current += 1;}

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.current += 1;
        }

        while self.peek().is_digit(10) {self.current += 1;}

        //add_tokenLiteral expects a literal of type Object
        match self.substring(self.start, self.current).parse::<f64>() {
            Ok(num) => {
                self.add_token_literal(TokenType::NUMBER,Object::NUMBER(num));
            }
            Err(_) => (),
        }
    }

    fn identifier(&mut self) -> () {
        while self.peek().is_alphanumeric() {
            self.advance();
        }

        let text= &self.source[self.start..self.current];

        let token_type = KEYWORDS.get(text)
            .cloned()
            .unwrap_or(TokenType::IDENTIFIER);

        self.add_token(token_type);
    }
}
