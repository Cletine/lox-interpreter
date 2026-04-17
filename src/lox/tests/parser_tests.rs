use super::super::*;

#[test]
fn empty_primary() {
    let test_tokens = vec![Token {token_type: TokenType::EOF, lexeme: "".to_string(), literal: Object::NULL, line: 1}];
    let parser = LoxParser{
        tokens: test_tokens, current: 0 
    };

    assert_eq!(parser.parse(), Expr::Literal {value: Object::STRING("".to_string()), });
}
