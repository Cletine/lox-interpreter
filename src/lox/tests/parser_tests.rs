use super::super::*;

#[test]
fn nil_primary() {
    let test_tokens = vec![Token { token_type: TokenType::NIL, lexeme: "nill".to_string(), literal: Object::NULL, line: 1 }, Token {token_type: TokenType::EOF, lexeme: "".to_string(), literal: Object::NULL, line: 1}];
    let mut parser = LoxParser{
        tokens: test_tokens, current_index: 0 
    };

    assert_eq!(parser.parse(), Ok(Expr::Literal {value: Object::NULL, }));
}

fn number_primary() {
    let test_tokens = vec![Token { token_type: TokenType::NUMBER, lexeme: "1.0".to_string(), literal: Object::NUMBER(1.0), line: 1 }, Token {token_type: TokenType::EOF, lexeme: "".to_string(), literal: Object::NULL, line: 1}];
    let mut parser = LoxParser{
        tokens: test_tokens, current_index: 0 
    };

    assert_eq!(parser.parse(), Ok(Expr::Literal {value: Object::NUMBER(1.0), }));
}
fn string_primary() {
    let test_tokens = vec![Token { token_type: TokenType::STRING, lexeme: "String".to_string(), literal: Object::NUMBER(1.0), line: 1 }, Token {token_type: TokenType::EOF, lexeme: "".to_string(), literal: Object::NULL, line: 1}];
    let mut parser = LoxParser{
        tokens: test_tokens, current_index: 0 
    };

    assert_eq!(parser.parse(), Ok(Expr::Literal {value: Object::STRING("String".to_string()), }));
}

fn true_primary() {
    let test_tokens = vec![Token { token_type: TokenType::TRUE, lexeme: "true".to_string(), literal: Object::NULL, line: 1 }, Token {token_type: TokenType::EOF, lexeme: "".to_string(), literal: Object::NULL, line: 1}];
    let mut parser = LoxParser{
        tokens: test_tokens, current_index: 0 
    };

    assert_eq!(parser.parse(), Ok(Expr::Literal {value: Object::BOOL(true), }));
}

fn false_primary() {
    let test_tokens = vec![Token { token_type: TokenType::TRUE, lexeme: "false".to_string(), literal: Object::NULL, line: 1 }, Token {token_type: TokenType::EOF, lexeme: "".to_string(), literal: Object::NULL, line: 1}];
    let mut parser = LoxParser{
        tokens: test_tokens, current_index: 0 
    };

    assert_eq!(parser.parse(), Ok(Expr::Literal {value: Object::BOOL(false), }));
}

fn expression_primary() {
    let test_tokens = vec![Token { token_type: TokenType::LeftParen, lexeme: "(".to_string(), literal: Object::NULL, line: 1 }, 
                           Token { token_type: TokenType::NUMBER, lexeme: "1.0".to_string(), literal: Object::NUMBER(1.0), line: 1 }, 
                           Token { token_type: TokenType::RightParen, lexeme: ")".to_string(), literal: Object::NULL, line: 1 }, 
                           Token {token_type: TokenType::EOF, lexeme: "".to_string(), literal: Object::NULL, line: 1}];
    let mut parser = LoxParser{
        tokens: test_tokens, current_index: 0 
    };

    assert_eq!(parser.parse(), Ok(Expr::Grouping {expression: Box::new(Expr::Literal {value: Object::NUMBER(1.0), }), }));
}

fn neg_unary () {
    let test_tokens = vec![Token { token_type: TokenType::Minus, lexeme: "-".to_string(), literal: Object::NULL, line: 1 }, 
                           Token { token_type: TokenType::NUMBER, lexeme: "1.0".to_string(), literal: Object::NUMBER(1.0), line: 1 }, 
                           Token {token_type: TokenType::EOF, lexeme: "".to_string(), literal: Object::NULL, line: 1}];
    let mut parser = LoxParser{
        tokens: test_tokens, current_index: 0 
    };

    assert_eq!(parser.parse(), Ok(Expr::Unary{operator: Token { token_type: TokenType::Minus, lexeme: "-".to_string(), literal: Object::NULL, line: 1 }, 
                                        right: Box::new(Expr::Literal {value: Object::NUMBER(1.0), }),}));

}

fn bang_unary () {
    let test_tokens = vec![Token { token_type: TokenType::Bang, lexeme: "!".to_string(), literal: Object::NULL, line: 1 }, 
                        Token { token_type: TokenType::NUMBER, lexeme: "1.0".to_string(), literal: Object::NUMBER(1.0), line: 1 }, 
                        Token {token_type: TokenType::EOF, lexeme: "".to_string(), literal: Object::NULL, line: 1}];
    let mut parser = LoxParser{
        tokens: test_tokens, current_index: 0 
    };

    assert_eq!(parser.parse(), Ok(Expr::Unary{operator: Token { token_type: TokenType::Bang, lexeme: "!".to_string(), literal: Object::NULL, line: 1 },
                                            right: Box::new(Expr::Literal {value: Object::NUMBER(1.0), }),}));
}

fn div_factor() {
    let test_tokens = vec![Token { token_type: TokenType::NUMBER, lexeme: "1".to_string(), literal: Object::NUMBER(1.0), line: 1 }, 
                           Token { token_type: TokenType::Slash, lexeme: "/".to_string(), literal: Object::NULL, line: 1 }, 
                           Token { token_type: TokenType::NUMBER, lexeme: "2".to_string(), literal: Object::NUMBER(2.0), line: 1 }, 
                           Token {token_type: TokenType::EOF, lexeme: "".to_string(), literal: Object::NULL, line: 1}];
    let mut parser = LoxParser{
        tokens: test_tokens, current_index: 0 
    };

    assert_eq!(parser.parse(), Ok(Expr::Binary{
                                            left: Box::new(Expr::Literal {value: Object::NUMBER(1.0), }),
                                            operator: Token { token_type: TokenType::Slash, lexeme: "/".to_string(), literal: Object::NULL, line: 1 },
                                            right: Box::new(Expr::Literal {value: Object::NUMBER(2.0), }),}));

}

fn mul_factor() {
    let test_tokens = vec![Token { token_type: TokenType::NUMBER, lexeme: "1".to_string(), literal: Object::NUMBER(1.0), line: 1 }, 
                           Token { token_type: TokenType::Star, lexeme: "*".to_string(), literal: Object::NULL, line: 1 }, 
                           Token { token_type: TokenType::NUMBER, lexeme: "2".to_string(), literal: Object::NUMBER(2.0), line: 1 }, 
                           Token {token_type: TokenType::EOF, lexeme: "".to_string(), literal: Object::NULL, line: 1}];
    let mut parser = LoxParser{
        tokens: test_tokens, current_index: 0 
    };

    assert_eq!(parser.parse(), Ok(Expr::Binary{
                                            left: Box::new(Expr::Literal {value: Object::NUMBER(1.0), }),
                                            operator: Token { token_type: TokenType::Star, lexeme: "*".to_string(), literal: Object::NULL, line: 1 },
                                            right: Box::new(Expr::Literal {value: Object::NUMBER(2.0), }),}));
}

fn add_terms() {
    let test_tokens = vec![Token { token_type: TokenType::NUMBER, lexeme: "1".to_string(), literal: Object::NUMBER(1.0), line: 1 }, 
                           Token { token_type: TokenType::Plus, lexeme: "+".to_string(), literal: Object::NULL, line: 1 }, 
                           Token { token_type: TokenType::NUMBER, lexeme: "2".to_string(), literal: Object::NUMBER(2.0), line: 1 }, 
                           Token {token_type: TokenType::EOF, lexeme: "".to_string(), literal: Object::NULL, line: 1}];
    let mut parser = LoxParser{
        tokens: test_tokens, current_index: 0 
    };

    assert_eq!(parser.parse(), Ok(Expr::Binary{
                                            left: Box::new(Expr::Literal {value: Object::NUMBER(1.0), }),
                                            operator: Token { token_type: TokenType::Plus, lexeme: "+".to_string(), literal: Object::NULL, line: 1 },
                                            right: Box::new(Expr::Literal {value: Object::NUMBER(2.0), }),}));
}

fn sub_terms() {
    let test_tokens = vec![Token { token_type: TokenType::NUMBER, lexeme: "1".to_string(), literal: Object::NUMBER(1.0), line: 1 }, 
                           Token { token_type: TokenType::Minus, lexeme: "-".to_string(), literal: Object::NULL, line: 1 }, 
                           Token { token_type: TokenType::NUMBER, lexeme: "2".to_string(), literal: Object::NUMBER(2.0), line: 1 }, 
                           Token {token_type: TokenType::EOF, lexeme: "".to_string(), literal: Object::NULL, line: 1}];
    let mut parser = LoxParser{
        tokens: test_tokens, current_index: 0 
    };

    assert_eq!(parser.parse(), Ok(Expr::Binary{
                                            left: Box::new(Expr::Literal {value: Object::NUMBER(1.0), }),
                                            operator: Token { token_type: TokenType::Minus, lexeme: "-".to_string(), literal: Object::NULL, line: 1 },
                                            right: Box::new(Expr::Literal {value: Object::NUMBER(2.0), }),}));
}

fn factored_term() {
    let test_tokens = vec![Token { token_type: TokenType::Minus, lexeme: "-".to_string(), literal: Object::NULL, line: 1 },
                           Token { token_type: TokenType::NUMBER, lexeme: "1".to_string(), literal: Object::NUMBER(1.0), line: 1 }, 
                           Token { token_type: TokenType::Plus, lexeme: "+".to_string(), literal: Object::NULL, line: 1 }, 
                           Token { token_type: TokenType::NUMBER, lexeme: "2".to_string(), literal: Object::NUMBER(2.0), line: 1 }, 
                           Token {token_type: TokenType::EOF, lexeme: "".to_string(), literal: Object::NULL, line: 1}];
    let mut parser = LoxParser{
        tokens: test_tokens, current_index: 0 
    };

    assert_eq!(parser.parse(), Ok(Expr::Binary{
                                            left: Box::new(Expr::Unary{operator: Token { token_type: TokenType::Minus, lexeme: "-".to_string(), literal: Object::NULL, line: 1 }, 
                                                              right: Box::new(Expr::Literal {value: Object::NUMBER(1.0), }),}),
                                            operator: Token { token_type: TokenType::Plus, lexeme: "+".to_string(), literal: Object::NULL, line: 1 },
                                            right: Box::new(Expr::Literal {value: Object::NUMBER(2.0), }),}));
}

fn unary_after_term() {
    let test_tokens = vec![Token { token_type: TokenType::Minus, lexeme: "-".to_string(), literal: Object::NULL, line: 1 },
                           Token { token_type: TokenType::NUMBER, lexeme: "1".to_string(), literal: Object::NUMBER(1.0), line: 1 }, 
                           Token { token_type: TokenType::Plus, lexeme: "+".to_string(), literal: Object::NULL, line: 1 }, 
                           Token { token_type: TokenType::Minus, lexeme: "-".to_string(), literal: Object::NULL, line: 1 },
                           Token { token_type: TokenType::NUMBER, lexeme: "2".to_string(), literal: Object::NUMBER(2.0), line: 1 }, 
                           Token {token_type: TokenType::EOF, lexeme: "".to_string(), literal: Object::NULL, line: 1}];
    let mut parser = LoxParser{
        tokens: test_tokens, current_index: 0 
    };

    assert_eq!(parser.parse(), Ok(Expr::Binary{
                                            left: Box::new(Expr::Unary{operator: Token { token_type: TokenType::Minus, lexeme: "-".to_string(), literal: Object::NULL, line: 1 }, 
                                                              right: Box::new(Expr::Literal {value: Object::NUMBER(1.0), }),}),
                                            operator: Token { token_type: TokenType::Plus, lexeme: "+".to_string(), literal: Object::NULL, line: 1 },
                                            right: Box::new(Expr::Unary{operator: Token { token_type: TokenType::Minus, lexeme: "-".to_string(), literal: Object::NULL, line: 1 }, 
                                                              right: Box::new(Expr::Literal {value: Object::NUMBER(2.0), }),}),}));
}

fn less_than_comp() {
    let test_tokens = vec![Token { token_type: TokenType::NUMBER, lexeme: "1".to_string(), literal: Object::NUMBER(1.0), line: 1 }, 
                           Token { token_type: TokenType::Less, lexeme: "<".to_string(), literal: Object::NULL, line: 1 }, 
                           Token { token_type: TokenType::NUMBER, lexeme: "2".to_string(), literal: Object::NUMBER(2.0), line: 1 }, 
                           Token {token_type: TokenType::EOF, lexeme: "".to_string(), literal: Object::NULL, line: 1}];
    let mut parser = LoxParser{
        tokens: test_tokens, current_index: 0 
    };

    assert_eq!(parser.parse(), Ok(Expr::Binary{
                                            left:  Box::new(Expr::Literal {value: Object::NUMBER(1.0), }),
                                            operator: Token { token_type: TokenType::Less, lexeme: "<".to_string(), literal: Object::NULL, line: 1 },
                                            right:  Box::new(Expr::Literal {value: Object::NUMBER(2.0), }),}),);
}

fn less_than_eq_comp() {
    let test_tokens = vec![Token { token_type: TokenType::NUMBER, lexeme: "1".to_string(), literal: Object::NUMBER(1.0), line: 1 }, 
                           Token { token_type: TokenType::LessEqual, lexeme: "<=".to_string(), literal: Object::NULL, line: 1 }, 
                           Token { token_type: TokenType::NUMBER, lexeme: "2".to_string(), literal: Object::NUMBER(2.0), line: 1 }, 
                           Token {token_type: TokenType::EOF, lexeme: "".to_string(), literal: Object::NULL, line: 1}];
    let mut parser = LoxParser{
        tokens: test_tokens, current_index: 0 
    };

    assert_eq!(parser.parse(), Ok(Expr::Binary{
                                            left:  Box::new(Expr::Literal {value: Object::NUMBER(1.0), }),
                                            operator: Token { token_type: TokenType::LessEqual, lexeme: "<=".to_string(), literal: Object::NULL, line: 1 },
                                            right:  Box::new(Expr::Literal {value: Object::NUMBER(2.0), }),}),);
}

fn greater_than_comp() {
    let test_tokens = vec![Token { token_type: TokenType::NUMBER, lexeme: "1".to_string(), literal: Object::NUMBER(1.0), line: 1 }, 
                           Token { token_type: TokenType::Greater, lexeme: ">".to_string(), literal: Object::NULL, line: 1 }, 
                           Token { token_type: TokenType::NUMBER, lexeme: "2".to_string(), literal: Object::NUMBER(2.0), line: 1 }, 
                           Token {token_type: TokenType::EOF, lexeme: "".to_string(), literal: Object::NULL, line: 1}];
    let mut parser = LoxParser{
        tokens: test_tokens, current_index: 0 
    };

    assert_eq!(parser.parse(), Ok(Expr::Binary{
                                            left:  Box::new(Expr::Literal {value: Object::NUMBER(1.0), }),
                                            operator: Token { token_type: TokenType::Greater, lexeme: ">".to_string(), literal: Object::NULL, line: 1 },
                                            right:  Box::new(Expr::Literal {value: Object::NUMBER(2.0), }),}),);
}

fn greater_than_eq_comp() {
    let test_tokens = vec![Token { token_type: TokenType::NUMBER, lexeme: "1".to_string(), literal: Object::NUMBER(1.0), line: 1 }, 
                           Token { token_type: TokenType::GreaterEqual, lexeme: ">=".to_string(), literal: Object::NULL, line: 1 }, 
                           Token { token_type: TokenType::NUMBER, lexeme: "2".to_string(), literal: Object::NUMBER(2.0), line: 1 }, 
                           Token {token_type: TokenType::EOF, lexeme: "".to_string(), literal: Object::NULL, line: 1}];
    let mut parser = LoxParser{
        tokens: test_tokens, current_index: 0 
    };

    assert_eq!(parser.parse(), Ok(Expr::Binary{
                                            left:  Box::new(Expr::Literal {value: Object::NUMBER(1.0), }),
                                            operator: Token { token_type: TokenType::GreaterEqual, lexeme: ">=".to_string(), literal: Object::NULL, line: 1 },
                                            right:  Box::new(Expr::Literal {value: Object::NUMBER(2.0), }),}),);
}

fn equal_comp() {
    let test_tokens = vec![Token { token_type: TokenType::NUMBER, lexeme: "1".to_string(), literal: Object::NUMBER(1.0), line: 1 }, 
                           Token { token_type: TokenType::EqualEqual, lexeme: "==".to_string(), literal: Object::NULL, line: 1 }, 
                           Token { token_type: TokenType::NUMBER, lexeme: "2".to_string(), literal: Object::NUMBER(2.0), line: 1 }, 
                           Token {token_type: TokenType::EOF, lexeme: "".to_string(), literal: Object::NULL, line: 1}];
    let mut parser = LoxParser{
        tokens: test_tokens, current_index: 0 
    };

    assert_eq!(parser.parse(), Ok(Expr::Binary{
                                            left:  Box::new(Expr::Literal {value: Object::NUMBER(1.0), }),
                                            operator: Token { token_type: TokenType::EqualEqual, lexeme: "==".to_string(), literal: Object::NULL, line: 1 },
                                            right:  Box::new(Expr::Literal {value: Object::NUMBER(2.0), }),}),);
}

fn not_equal_comp() {
    let test_tokens = vec![Token { token_type: TokenType::NUMBER, lexeme: "1".to_string(), literal: Object::NUMBER(1.0), line: 1 }, 
                           Token { token_type: TokenType::BangEqual, lexeme: "!=".to_string(), literal: Object::NULL, line: 1 }, 
                           Token { token_type: TokenType::NUMBER, lexeme: "2".to_string(), literal: Object::NUMBER(2.0), line: 1 }, 
                           Token {token_type: TokenType::EOF, lexeme: "".to_string(), literal: Object::NULL, line: 1}];
    let mut parser = LoxParser{
        tokens: test_tokens, current_index: 0 
    };

    assert_eq!(parser.parse(), Ok(Expr::Binary{
                                            left:  Box::new(Expr::Literal {value: Object::NUMBER(1.0), }),
                                            operator: Token { token_type: TokenType::BangEqual, lexeme: "!=".to_string(), literal: Object::NULL, line: 1 },
                                            right:  Box::new(Expr::Literal {value: Object::NUMBER(2.0), }),}),);
}


