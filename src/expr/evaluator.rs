use std::collections::HashMap;

use super::parser::{Expr, Parser};

pub type Vars = HashMap<String, Evalue>;

pub struct Evaluator {}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Evalue {
    String(String),
    Number(f64),
    Bool(bool),
}
impl Evalue {
    fn to_bool(&self) -> bool {
        match self {
            Evalue::String(v) => v != "" && v != "0",
            Evalue::Number(n) => *n != 0.0,
            Evalue::Bool(b) => *b,
        }
    }

    fn to_string(&self) -> String {
        match self {
            Evalue::String(v) => v.clone(),
            Evalue::Number(n) => format!("{}", *n),
            Evalue::Bool(b) => match b {
                true => "true".to_string(),
                false => "false".to_string(),
            },
        }
    }
}

impl Evaluator {
    pub fn new() -> Evaluator {
        Evaluator {}
    }

    pub fn parse(&mut self, expr: &str) -> Result<Expr, String> {
        Parser::new(expr).parse()
    }

    pub fn parse_and_evaluate(&mut self, expr: &str, vars: &Vars) -> Result<bool, String> {
        let expr = Parser::new(expr).parse()?;
        self.evaluate(&expr, vars)
    }

    pub fn evaluate(&self, expr: &Expr, vars: &Vars) -> Result<bool, String> {
        match self.evaluate_expr(&expr, vars)? {
            Evalue::String(_) | Evalue::Number(_) => {
                Err(format!("expression must evluate to a boolean, got: {:?}", expr).to_string())
            }
            Evalue::Bool(b) => Ok(b),
        }
    }

    fn evaluate_expr(&self, expr: &super::parser::Expr, vars: &Vars) -> Result<Evalue, String> {
        match expr {
            super::parser::Expr::Boolean(b) => Ok(Evalue::Bool(*b)),
            super::parser::Expr::String(s) => Ok(Evalue::String(s.clone())),
            super::parser::Expr::Binary(lexpr, op, rexpr) => {
                let lval = self.evaluate_expr(lexpr, vars)?;
                let rval = self.evaluate_expr(rexpr, vars)?;
                let eval = match op {
                    super::lexer::TokenKind::GreaterThan => Ok(lval > rval),
                    super::lexer::TokenKind::GreaterThanEqual => Ok(lval >= rval),
                    super::lexer::TokenKind::LessThanEqual => Ok(lval <= rval),
                    super::lexer::TokenKind::LessThan => Ok(lval < rval),
                    super::lexer::TokenKind::Equal => Ok(lval == rval),
                    super::lexer::TokenKind::FuzzyEqual => Ok(lval.to_string().contains(rval.to_string().as_str())),
                    super::lexer::TokenKind::NotEqual => Ok(lval != rval),
                    super::lexer::TokenKind::NotFuzzyEqual => Ok(!lval.to_string().contains(rval.to_string().as_str())),
                    super::lexer::TokenKind::Or => Ok(lval.to_bool() || rval.to_bool()),
                    super::lexer::TokenKind::And => Ok(lval.to_bool() && rval.to_bool()),
                    _ => Err(format!("unknown operator: {:?}", op)),
                }?;
                Ok(Evalue::Bool(eval))
            }
            super::parser::Expr::Number(n) => Ok(Evalue::Number(*n)),
            super::parser::Expr::Variable(v) => match vars.get(v) {
                Some(v) => Ok(v.clone()),
                None => Err(format!("Unknown variable `{}`", v)),
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_evaluate() {
        let result = Evaluator::new().parse_and_evaluate("false", &HashMap::new());
        assert_eq!(false, result.unwrap());
        let result = Evaluator::new().parse_and_evaluate("20 > 10", &HashMap::new());

        assert_eq!(true, result.unwrap());

        let result = Evaluator::new().parse_and_evaluate("20 > 10 and false", &HashMap::new());

        assert_eq!(false, result.unwrap());
    }

    #[test]
    fn test_evaluate_params() {
        let map = HashMap::from([
            ("distance".to_string(), Evalue::Number(10.0)),
            ("type".to_string(), Evalue::String("Run".to_string())),
        ]);
        let result = Evaluator::new().parse_and_evaluate("distance > 5", &map);
        assert_eq!(true, result.unwrap());
        let result = Evaluator::new().parse_and_evaluate("distance < 5", &map);
        assert_eq!(false, result.unwrap());
        let result = Evaluator::new().parse_and_evaluate("distance = 10", &map);
        assert_eq!(true, result.unwrap());
        let result = Evaluator::new().parse_and_evaluate("type = 'Run'", &map);
        assert_eq!(true, result.unwrap());
        let result = Evaluator::new().parse_and_evaluate("type ~ 'Ru'", &map);
        assert_eq!(true, result.unwrap());
        let result = Evaluator::new().parse_and_evaluate("type !~ 'Rup'", &map);
        assert_eq!(true, result.unwrap());
        let result = Evaluator::new().parse_and_evaluate("type != 'Run'", &map);
        assert_eq!(false, result.unwrap());
    }
}
