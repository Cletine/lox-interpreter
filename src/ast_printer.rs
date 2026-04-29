use crate::lox::Expr;
use crate::lox::Object;
use crate::lox::Token;
use crate::lox::TokenType;


pub fn print_ast(expr: &Expr) {
    let ast_as_string : String = evaluate_ast(expr);
    println!("{}", ast_as_string); 
}

fn evaluate_ast (expr: &Expr) -> String {
    match expr {
        Expr::Binary {left, operator, right}=> {
            let left_val = evaluate_ast(left);
            let right_val = evaluate_ast(right);
            format!("({} {} {})", operator.lexeme, left_val, right_val)
        }
        Expr::Unary{operator, right}=> {
            let right_val = evaluate_ast(right);
            format!("({} {})", operator.lexeme, right_val)
        }
        Expr::Grouping{expression}=> {
            let group_expr = evaluate_ast(expression);
            format!("(group {} )", group_expr)
        }
        Expr::Literal{value} => {
            format!("{}", value.object_to_string())
        }
    }
}
