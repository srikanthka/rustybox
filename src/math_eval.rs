use std::str::FromStr;
use thiserror::Error;

pub trait Evaluator {
    fn eval(&self, expr: &str) -> Result<f64, EvalError>;
}

///Error Type
#[derive(Error,Debug)]
pub enum EvalError {
    #[error("Parse error: {0}")]
    Parse(String),
    #[error("Invalid token: {0}")]
    InvalidToken(String),
    #[error("Empty expression")]
    Empty,
    #[error("Division by zero")]
    DivisionByZero,
}

pub struct SimpleEvaluator;

impl Evaluator for SimpleEvaluator {
    fn eval(&self, expr: &str) -> Result<f64, EvalError> {
        let trimmed = expr.trim();
        if trimmed.is_empty(){
            return Err(EvalError::Empty);
        }

        match meval::eval_str(trimmed) {
            Ok(result) => Ok(result),
            Err(e) => Err(EvalError::Parse(e.to_string())),
        }
    }
}

pub fn run(expr:&str){
    let evaluator = SimpleEvaluator;

    match evaluator.eval(expr){
        Ok(result) => {println!("Result: {}", result)}
        Err(e) => eprintln!(" Error: {}", e),
    }
}
