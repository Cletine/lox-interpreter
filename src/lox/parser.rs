use crate::lox::Token;
use crate::lox::TokenType;
use crate::lox::Object;
use crate::lox::Expr;
use crate::parse_error;

pub struct LoxParser {
    pub tokens: Vec<Token>,
    pub current: usize,
}


// TODO Implement a Result<Expr, ParseError> return for the parse
// This may prove to be essential for its function and testablity
impl LoxParser {

    fn parse(&mut self) {
        self.expression();
    }

    fn expression(&mut self) -> Expr {
        self.equality() 
    }

    fn equality(&mut self) -> Expr {
        let mut left_expr : Expr = self.comparision();
        loop {
            match self.tokens[self.current].token_type  {
                TokenType::BangEqual | TokenType::EqualEqual => {
                    let operator = self.tokens[self.current].clone();
                    self.current += 1;
                    let right_expr = self.comparision();

                    left_expr = Expr::Binary{
                        left: Box::new(left_expr),
                        operator: operator, 
                        right: Box::new(right_expr),
                    };
                }

                _ => break,
            }
        }

        left_expr 
    }

    fn comparision(&mut self) -> Expr {
        let mut left_term : Expr  = self.term();
        loop {
            match self.tokens[self.current].token_type  {
                TokenType::Greater | TokenType::Less | TokenType::GreaterEqual | TokenType::LessEqual => {
                    let operator = self.tokens[self.current].clone();
                    self.current += 1;
                    let right_term = self.term();

                    left_term = Expr::Binary{
                        left: Box::new(left_term),
                        operator: operator, 
                        right: Box::new(right_term),
                    };
                }

                _ => break,
            }
        }
        left_term
    }



    fn term(&mut self) -> Expr {
        let mut left_factor : Expr = self.factor();
        loop {
            match self.tokens[self.current].token_type  {
                TokenType::Minus | TokenType::Plus => {
                    let operator = self.tokens[self.current].clone();
                    self.current += 1;
                    let right_factor = self.factor();

                    left_factor = Expr::Binary{
                        left: Box::new(left_factor),
                        operator: operator, 
                        right: Box::new(right_factor),
                    };
                }

                _ => break,
            }
        }
        left_factor
    }

    fn factor(&mut self) -> Expr {
        let mut left_unary: Expr = self.unary();
        loop {
            match self.tokens[self.current].token_type  {
                TokenType::Slash | TokenType::Star => {
                    let operator = self.tokens[self.current].clone();
                    self.current += 1;
                    let right_unary = self.unary();

                    left_unary = Expr::Binary{
                        left: Box::new(left_unary),
                        operator: operator, 
                        right: Box::new(right_unary),
                    };
                }

                _ => break,
            }
        }

        left_unary
    }

    fn unary(&mut self) -> Expr {
        match  self.tokens[self.current].token_type {
            TokenType::Bang | TokenType::Minus => {
                let operator = self.tokens[self.current].clone();
                self.current += 1;
                let unary = self.unary();

                Expr::Unary {
                    operator: operator, 
                    right: Box::new(unary),
                }

            }

            _ => self.primary(),
        }

    }

    fn primary(&mut self) -> Expr {
        match self.tokens[self.current].token_type {
            TokenType::NUMBER => {
                Expr::Literal {
                    value: self.tokens[self.current].literal,
                }
            }
            TokenType::STRING => {
                Expr::Literal {
                    value: self.tokens[self.current].literal,
                }
            }
            TokenType::FALSE => {
                Expr::Literal {
                    value: Object::BOOL(false),
                }
            }
            TokenType::TRUE => {
                Expr::Literal {
                    value: Object::BOOL(true),
                }
            }
            TokenType::NIL => {
                Expr::Literal {
                    value: Object::NULL
                }
            }
            TokenType::LeftParen => {
                let expr = self.expression();
                self.current += 1;
                match self.tokens[self.current].token_type {
                    TokenType::RightParen => {
                        Expr::Grouping {
                            expression: Box::new(expr),
                        }

                    }

                    _ => parse_error(self.tokens[self.current], "Expect ')' after expression."),
                }
            }

            _ => parse_error(self.tokens[self.current], "Expect expression."),

        }
    }

    fn syncronize(&mut self) -> () {
        // move off the current error throwing token
        self.current += 1;

        while !self.is_at_end() {
            // checks if previous token was a statement terminator ';'
            if self.tokens[self.current - 1].token_type == TokenType::SemiColon {
                break
            }
            match self.tokens[self.current].token_type {
                TokenType::CLASS |
                    TokenType::FUN |
                    TokenType::FOR |
                    TokenType::IF |
                    TokenType::WHILE |
                    TokenType::PRINT |
                    TokenType::RETURN |
                    TokenType::VAR => break,
                    _ => self.current += 1,
            }
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len() 
    }


}
