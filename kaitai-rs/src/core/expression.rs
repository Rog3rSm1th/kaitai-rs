use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./core/expr.pest"]
struct ExprParser;

#[derive(Debug)]
pub struct ExpressionInterpreter {}

impl ExpressionInterpreter {
    pub fn new() -> Self {
        ExpressionInterpreter {}
    }

    pub fn evaluate(&self, _expr: &str) -> i32 {
        1
    }
}
