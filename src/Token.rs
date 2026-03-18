use TokenType

struct  Token {
    type: TokenType,
    lexeme: &str,
    literal:Object,
    line: u64;
}

impl Token {
    pub fn toString(&self) -> &'static str{
        "{self.type} {self.lexeme} {self.literal}"
    } 
}
