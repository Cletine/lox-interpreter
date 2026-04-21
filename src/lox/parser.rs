use crate::lox::Token;
use crate::lox::TokenType;
use crate::lox::Object;
use crate::lox::Expr;
use crate::parse_error;
use crate::ParserError;



// idea should be to handle error propagation when inside 
// recursive descent parser
//


pub struct LoxParser {
    pub tokens: Vec<Token>,
    pub current: usize,
}


// Initially we will have it return the first instance of an error  and as we add expression
// evaluation, we will syncronize the parser to break from error and continue to parse through the
// errored function.
// TODO Implement Syncornize to the parser
impl LoxParser {

        // TODO Assume additional functionality for the outputs of this function.
    fn parse(&mut self) -> Result<Expr, ParserError> {
        Self::expression(&mut self.tokens, &mut self.current)
    }

    // This function should parse through the entire expression and 
    // branch according to whether it is a sound expression or not 
    fn expression <'a> (tokens: &'a mut Vec<Token>, current: &'a mut usize) -> (Result<Expr, ParserError<'a>>, usize) {
        Self::equality(tokens, current)
    }

    // TODO Needs some sort of accumulator to collect the current count of recursive expressions (*)
    fn equality <'a> (tokens: &'a mut Vec<Token>, current: &'a mut usize) -> (Result<Expr, ParserError<'a>>, usize) {
        let mut parse_error_status: Option<_> = None;
        // Determine left sided soundness of the left side of the exression
        let left_expr : Result<Expr, ParserError> = 
            match Self::comparision(tokens, current) {
                Ok(mut left_expr) => Ok(left_expr),
                // return left sided parsing error 
                Err(parse_error) => Err(parse_error),
            };

        match left_expr {
            Ok(left_expr) => {
                loop {
                    match tokens[*current].token_type  {
                        TokenType::BangEqual | TokenType::EqualEqual => {
                            let operator = tokens[*current].clone();
                            *current += 1;

                            // Determine right sided soundness of the right side of the expression
                            match Self::comparision(tokens, current) {
                                // If both left and right are sound, return the expression
                                Ok(right_expr) => {
                                    left_expr = Expr::Binary{
                                        left: Box::new(left_expr),
                                        operator: operator, 
                                        right: Box::new(right_expr),
                                    }
                                }

                                // else return right sided parsing error
                                Err(parse_error) => { 
                                    parse_error_status = Some(parse_error);
                                    break
                                },
                            };
                        }
                        _ => break,
                    }
                }
                //Determines if any errors propagated back.
                match parse_error_status {
                    Some (parse_error) => Err(parse_error),
                    None => Ok(left_expr)
                }
            }
            Err(parse_error) => Err(parse_error),
        }

    }


    fn comparision <'a> (tokens: &'a mut Vec<Token>, current: &'a mut usize) -> Result<Expr, ParserError<'a>> {      
        // Determine left sided soundness of the left side of the exression
        let left_term: Result<Expr, ParserError> =
            match Self::term(tokens, current) {
                Ok(left_term) => {
                    let mut parse_error_status: Option<_> = None;
                    // Iterate through contiguous instances of the Toketype
                    loop {
                        match tokens[*current].token_type  {
                            TokenType::Greater | TokenType::Less | TokenType::GreaterEqual | TokenType::LessEqual => {
                                let operator = tokens[*current].clone();
                                *current += 1;
                                // Determine right sided soundness of the right side of the expression
                                match Self::term(tokens, current) {
                                    // If both left and right are sound, return the expression
                                    Ok(right_term) => {
                                        left_term = Expr::Binary{
                                            left: Box::new(left_term),
                                            operator: operator, 
                                            right: Box::new(right_term),
                                        }
                                    }
                                    // else return right sided parsing error
                                    Err(parse_error) =>  { 
                                        parse_error_status = Some(parse_error);
                                        break
                                    }
                                }
                            }
                            _ => break,
                        }
                    }
                    //Determines if right descent propagated any errors back.
                    match parse_error_status {
                        Some(parse_error) => Err(parse_error),
                        None => Ok(left_term)
                    }
                }
                // return left sided parsing error 
                Err(parse_error) => Err(parse_error),
            };

        return left_term
    }



    fn term <'a> (tokens: &'a mut Vec<Token>, current: &'a mut usize) -> Result<Expr, ParserError<'a>> {       
        // Determine left sided soundness of the left side of the exression
        let mut left_factor: Result<Expr, ParserError> = 
            match Self::factor(tokens, current) {
                Ok(left_factor) => {

                    let mut parse_error_status: Option<_> = None;
                    // Iterate through contiguous instances of the Toketype
                    loop {
                        match tokens[*current].token_type  {
                            TokenType::Minus | TokenType::Plus => {
                                let operator = tokens[*current].clone();
                                *current += 1;

                                // Determine right sided soundness of the right side of the expression
                                match Self::factor(tokens, current) {
                                    // If both left and right are sound, return the expression
                                    Ok(right_factor) => {
                                        left_factor = Expr::Binary{
                                            left: Box::new(left_factor),
                                            operator: operator, 
                                            right: Box::new(right_factor),
                                        };
                                    }
                                    // else return right sided parsing error
                                    Err(parse_error) => { 
                                        parse_error_status = Some(parse_error);
                                        break
                                    }
                                }
                            }
                            _ => break,
                        }
                    }
                    //Determines if right descent propagated any errors back.
                    match parse_error_status {
                        Some(parse_error) => Err(parse_error),
                        None => Ok(left_factor)
                    }            
                }
                // return left sided parsing error 
                Err(parse_error) => Err(parse_error),
            };

        return left_factor
    }


    fn factor<'a> (tokens: &'a mut Vec<Token>, current: &'a mut usize) -> Result<Expr, ParserError<'a>> {
        // Determine left sided soundness of the left side of the exression
        let mut left_unary: Result<Expr, ParserError> = match Self::unary(tokens, current) {
            Ok(left_unary) => {

                let mut parse_error_status: Option<_> = None;
                // Iterate through contiguous instances of the Toketype
                loop {
                    match tokens[*current].token_type  {
                        TokenType::Slash | TokenType::Star => {
                            let operator = tokens[*current].clone();
                            *current += 1;

                            // Determine right sided soundness of the right side of the expression
                            match Self::unary(tokens, current) {
                                // If both left and right are sound, return the expression
                                Ok(right_unary) => {
                                    left_unary= Expr::Binary{
                                        left: Box::new(left_unary),
                                        operator: operator, 
                                        right: Box::new(right_unary),
                                    };
                                }
                                // else return right sided parsing error
                                Err(parse_error) => { 
                                    parse_error_status = Some(parse_error);
                                    break
                                }
                            }
                        }
                        _ => break,
                    }
                }
                //Determines if right descent propagated any errors back.
                match parse_error_status {
                    Some(parse_error) => Err(parse_error),
                    None => Ok(left_unary)
                }           
            }
            // return left sided parsing error 
            Err(parse_error) => Err(parse_error),
        };

        return left_unary
    }


    fn unary <'a> (tokens: &'a mut Vec<Token>, current: &'a mut usize) -> Result<Expr, ParserError<'a>> {
        let unary: Result<Expr, ParserError> = 
            match tokens[*current].token_type  {
            TokenType::Slash | TokenType::Star => {
                let operator = tokens[*current].clone();
                *current += 1;

                match Self::unary(tokens, current) {
                    // If the expression is sound, return the expression
                    Ok(unary) => {
                        Ok(Expr::Unary {
                            operator: operator, 
                            right: Box::new(unary),
                        })     
                    }
                    // else return right sided parsing error
                    Err(parse_error) => Err(parse_error),
                }
            }
            _ => Self::primary(tokens, current),
        };

        return unary
    }


    fn primary<'a> (tokens: &'a mut Vec<Token>, current: &'a mut usize) -> Result<Expr, ParserError<'a>> {
        let primary: Result<Expr, ParserError> = 
            match tokens[*current].token_type {
            TokenType::NUMBER => {
                Ok(Expr::Literal {
                    value: tokens[*current].literal,
                })
            }
            TokenType::STRING => {
                Ok(Expr::Literal {
                    value: tokens[*current].literal,
                })
            }
            TokenType::FALSE => {
                Ok(Expr::Literal {
                    value: Object::BOOL(false),
                })
            }
            TokenType::TRUE => {
                Ok(Expr::Literal {
                    value: Object::BOOL(true),
                })
            }
            TokenType::NIL => {
                Ok(Expr::Literal {
                    value: Object::NULL
                })
            }
            TokenType::LeftParen => {
                *current += 1;
                let expr : Result<Expr, ParserError> = 
                    match Self::expression(tokens, current) {
                    Ok(expr) => {
                        *current += 1;
                        match tokens[*current].token_type {
                            TokenType::RightParen => {
                                Ok(Expr::Grouping {
                                    expression: Box::new(expr),
                                })
                            }
                            _ => {
                                //parse_error(self.tokens[self.current], "Expect ')' after expression.");
                                //TODO some syncronizing effort or panic to evaluate the entire ast 
                                Err(ParserError {error_msg: "Expect ')' after expression.", error_token: tokens[*current] })
                            }
                        }
                    }
                    Err(parse_error) => Err(parse_error),
                };
                return expr
            }
            _ => {  
                // parse_error(self.tokens[self.current], "Expect expression.");
                //TODO some syncronizing effort or panic to evaluate the entire ast 
                Err(ParserError {error_msg: "Expect expression", error_token: tokens[*current]})
            }
        };

        return primary
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
