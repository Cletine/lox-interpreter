use crate::lox::Token;
use crate::lox::Object;


#[derive (Debug, PartialEq, Clone)]
pub enum Expr {
	Binary {
    left: Box<Expr>,
    operator: Token,
    right: Box<Expr>,
    },
	Grouping {
    expression: Box<Expr>,
    },
	Literal {
    value: Object,
    },
	Unary {
    operator: Token,
    right: Box<Expr>,
    },
}
