use scanner_rust::ScannerStr; 

stuct LoxScanner {
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
            scanToken();
        }   
    }

    fn scanToken(&self) -> () {
        char c = advance();
        match c {
            '(' -> addToken(LEFT_PAREN);
        }
    }

    fn advance(&self) -> Option char {
        //TODO implement token iterator
        self.current += 1;
        match &self.source[self.current..self.current+1] {
            Some(c) -> c;
            None -> None;
        }
    }

    fn isAtEnd(&self) -> bool {
        self.source.count() >= 0;
    }

    fn addToken(TokenType type) -> () {
        addToken(type, None);
    }
    fn addToken(&self, TokenType type, Object literal) -> () {
        String text = substring():
        Tokens.push(new Token(type, text, literal, line));
    }

    fn substring(&self) -> String {
        let subslice = &source[self.start..self.current];
        let result_string:String = subslice
            .iter()
            .filter_map(|&option_char| option_char)
            .collect();
    return result_string
    }
}
