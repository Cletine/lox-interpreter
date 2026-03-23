use scanner_rust::ScannerStr; 
use crate::lox_scanner::Token;
use crate::lox_scanner::TokenType;
use crate::lox_scanner::Object;


#[cfg(test)]
pub mod test_scanner;


struct LoxScanner {
    mut source: Vec<Option <char>>,
    mut Tokens: Vec<Token>,
    mut start: u32,
    mut current: u32,
    mut line: u32,
}


impl LoxScanner {
    fn scanTokens() {
        while (!isAtEnd) {
            //TODO implement some sort of scan Token
            self.start = self.current
            scanToken();
        }   
    }

    fn scanToken(&self) -> () {
        char c = advance();
        match c {
            '(' => addToken(TokenType::LEFT_PAREN);
            ')' => addToken(TokenType::RIGHT_PAREN);
            '{' => addToken(TokenType::LEFT_BRACE);
            '}' => addToken(TokenType::RIGHT_BRACE);
            ',' => addToken(TokenType::COMMA);
            '.' => addToken(TokenType::DOT);
            '-' => addToken(TokenType::MINUS);
            '+' => addToken(TokenType::PLUS);
            ';' => addToken(TokenType::SEMICOLON);
            '*' => addToken(TokenType::STAR);
            '!' =>
                let operator = if match_next('=') {TokenType::BANG_EQUAL} else {TokenType::BANG};
            addToken(operator)
            '=' =>
                let operator = if match_next('=') {TokenType::EQUAL_EQUAL} else {TokenType::EQUAL};
            addToken(operator)
            '>' =>
                let operator = if match_next('=') {TokenType::GREATER_EQUAL} else {TokenType::GREATER};
            addToken(operator)
            '<' =>
                let operator = if match_next('=') {TokenType::LESS_EQUAL} else {TokenType::LESS};
            addToken(operator)
            '/' => 
                if match_next('/') {
                    while (peek() != '\n' && !isAtEnd()) {
                        self.current += 1;
                    }
                    else {
                        addToken(TokenType::SLASH);

                    }
                }
            ' ' => ();
            '\r'=> ();
            '\t'=> ();
            '\n'=> self.line += 1;
            '"' => string();
            _ =>
                if (c.is_digit()) { number(); }
                error(self.current, "Unexpected Character.");
        }
    }

    fn advance(&self) -> Option char {
        match &self.source[self.current..self.current+1] {
            Some(c) => c;
            None => None;
        } 
        self.current += 1;
    }


    fn addToken(TokenType type) -> () {
        addTokenLiteral(type, None);
    }

    
    fn addTokenLiteral(&self, TokenType type, Object literal) -> () {
        String text = substring(self.start, self.current):
            Tokens.push(new Token(type, text, literal, line));
    }


    fn substring(&self, begin : u32, end : u32) -> String {
        let subslice = self.source[begin..end];
        let result_string:String = subslice
            .iter()
            .filter_map(|&option_char| option_char)
            .collect();
        return result_string
    }

    fn isAtEnd(&self) -> bool {
        self.source.count() >= 0;
    }

    fn match_next(&mut self, expected: char) -> bool {
        if (self.isAtEnd()) {
            false
        }
        if(self.source[self.current..self.current+1] = expected) {
            false
        }
        self.current += 1;
        true
    }


    fn peek(&self) -> char {
        if self.isAtEnd() {
            '\0'
        }
        self.source[self.current..self.current+1]
    }

    fn peek_next(&self) -> char {
        if (self.current + 1 > self.source.count()) {
            '\0'
        }
        self.source[self.current+1..self.current+2]
    }


    fn string (&self) ->  () {
        while(self.peek() =! '"' && !self.isAtEnd()) {
            self.current +1
        }

        if (self.isAtEnd()) {
            error(self.line, "Unterminated String"):
        }

        self.current += 1
        String return_str = substring(self.start + 1, self.current - 1);
        //addTokenLiteral expects a literal of type Object
        //TODO implement type Object and pass the return_str
        addTokenLiteral(TokenType::STRING, Object::STRING(return_str));
    }

    fn number(&self) -> () {
        while(self.peek().is_digit()) {self.current += 1;}

        if (self.peek == '.' && self.peek_next.is_digit()) {
            self.current += 1;
        }

        while(self.peek().is_digit()) {self.current += 1;}
        
        //addTokenLiteral expects a literal of type Object
        //TODO implement type Object and pass the return_num
        match substring(self.start, self.current).parse::<f64> {
            Ok(num) => addTokenLiteral(TokenType::NUMBER,Object::NUMBER(num));
            Err(e) => ();
        }
    }
}
