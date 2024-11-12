use std::collections::HashMap;

use chrono::NaiveDate;

use super::{parser::{Expr, Parser}, lexer::TokenKind};

pub type Vars = HashMap<String, Evalue>;

pub struct Evaluator {}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Evalue {
    String(String),
    Number(f64),
    Bool(bool),
    Date(NaiveDate),
}
impl Evalue {
    fn to_bool(&self) -> bool {
        match self {
            Evalue::String(v) => v != "" && v != "0",
            Evalue::Number(n) => *n != 0.0,
            Evalue::Bool(b) => *b,
            Evalue::Date(_) => true,
        }
    }

    fn to_string(&self) -> String {
        match self {
            Evalue::Date(d) => d.to_string(),
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
            Evalue::Number(n) => {
                Err(format!("expression must evluate to a boolean, got {:?}: {:?}", expr, n).to_string())
            }
            Evalue::Date(_) | Evalue::String(_) | Evalue::Number(_) => {
                Err(format!("expression must evluate to a boolean, got: {:?}", expr).to_string())
            }
            Evalue::Bool(b) => Ok(b),
        }
    }

    fn evaluate_expr(&self, expr: &super::parser::Expr, vars: &Vars) -> Result<Evalue, String> {
        match expr {
            Expr::Boolean(b) => Ok(Evalue::Bool(*b)),
            Expr::String(s) => Ok(Evalue::String(s.clone())),
            Expr::Date(s) => Ok(Evalue::Date(s.clone())),
            Expr::Quantity(expr, unit) => {
                let val = match self.evaluate_expr(expr, vars)? {
                    Evalue::Number(n) => Ok(n),
                    _ =>  Err("Value must be numeric"),
                }?;
                Ok(Evalue::Number(unit.convert(val)))
            }
            Expr::Binary(lexpr, op, rexpr) => {
                let lval = self.evaluate_expr(lexpr, vars)?;
                let rval = self.evaluate_expr(rexpr, vars)?;
                let eval = match op {
                    TokenKind::GreaterThan => Ok(lval > rval),
                    TokenKind::GreaterThanEqual => Ok(lval >= rval),
                    TokenKind::LessThanEqual => Ok(lval <= rval),
                    TokenKind::LessThan => Ok(lval < rval),
                    TokenKind::Equal => Ok(lval == rval),
                    TokenKind::FuzzyEqual => Ok(lval.to_string().contains(rval.to_string().as_str())),
                    TokenKind::NotEqual => Ok(lval != rval),
                    TokenKind::NotFuzzyEqual => Ok(!lval.to_string().contains(rval.to_string().as_str())),
                    TokenKind::Or => Ok(lval.to_bool() || rval.to_bool()),
                    TokenKind::And => Ok(lval.to_bool() && rval.to_bool()),
                    _ => Err(format!("unknown operator: {:?}", op)),
                }?;
                Ok(Evalue::Bool(eval))
            }
            Expr::Number(n) => Ok(Evalue::Number(*n)),
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
        let result = Evaluator::new().parse_and_evaluate("2024-01-06 > 2020-01-06", &map);
        assert_eq!(true, result.unwrap());
        let result = Evaluator::new().parse_and_evaluate("2024-01-06 < 2020-01-06", &map);
        assert_eq!(false, result.unwrap());
        let result = Evaluator::new().parse_and_evaluate("1kmph = 1000", &map);
        assert_eq!(true, result.unwrap());
        let result = Evaluator::new().parse_and_evaluate("1mph > 1609 and 1mph < 1610", &map);
        assert_eq!(true, result.unwrap());
    }
}
