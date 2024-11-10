use std::collections::HashMap;

use super::parser::Parser;

struct Evaluator<'a, 'b> {
    parser: Parser<'a>,
    vars: &'b HashMap<String, Evalue>,
}

#[derive(PartialEq, Eq, PartialOrd, Debug, Clone)]
enum Evalue {
    String(String),
    Number(u16),
    Bool(bool),
}
impl Evalue {
    fn to_bool(&self) -> bool {
        match self {
            Evalue::String(v) => v != "" && v != "0",
            Evalue::Number(n) => *n != 0,
            Evalue::Bool(b) => *b,
        }
    }
}

impl Evaluator<'_, '_> {
    pub fn new<'a, 'b>(expr: &'a str, vars: &'b HashMap<String, Evalue>) -> Evaluator<'a, 'b> {
        Evaluator {
            parser: Parser::new(expr),
            vars,
        }
    }

    pub fn evaluate(&mut self) -> Result<bool, String> {
        let expr = self.parser.parse()?;
        match self.evaluate_expr(expr.clone())? {
            Evalue::String(_) | Evalue::Number(_) => {
                Err(format!("expression must evluate to a boolean, got: {:?}", expr).to_string())
            }
            Evalue::Bool(b) => Ok(b),
        }
    }

    fn evaluate_expr(&self, expr: super::parser::Expr) -> Result<Evalue, String> {
        match expr {
            super::parser::Expr::Boolean(b) => Ok(Evalue::Bool(b)),
            super::parser::Expr::Binary(lexpr, op, rexpr) => {
                let lval = self.evaluate_expr(*lexpr)?;
                let rval = self.evaluate_expr(*rexpr)?;
                let eval = match op {
                    super::lexer::TokenKind::GreaterThan => Ok(lval > rval),
                    super::lexer::TokenKind::GreaterThanEqual => Ok(lval >= rval),
                    super::lexer::TokenKind::LessThanEqual => Ok(lval <= rval),
                    super::lexer::TokenKind::LessThan => Ok(lval < rval),
                    super::lexer::TokenKind::Equal => Ok(lval == rval),
                    super::lexer::TokenKind::Or => Ok(lval.to_bool() || rval.to_bool()),
                    super::lexer::TokenKind::And => Ok(lval.to_bool() && rval.to_bool()),
                    _ => Err(format!("unknown operator: {:?}", op)),
                }?;
                Ok(Evalue::Bool(eval))
            }
            super::parser::Expr::Number(n) => Ok(Evalue::Number(n)),
            super::parser::Expr::Variable(v) => match self.vars.get(&v) {
                Some(v) => Ok(v.clone()),
                None => Err(format!("Unknown variable `{}`", v)),
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct TestSubject {
        distance: u64,
    }

    #[test]
    fn test_evaluate() {
        let result = Evaluator::new("false", &HashMap::new()).evaluate();
        assert_eq!(false, result.unwrap());
        let result = Evaluator::new("20 > 10", &HashMap::new()).evaluate();

        assert_eq!(true, result.unwrap());

        let result = Evaluator::new("20 > 10 and false", &HashMap::new()).evaluate();

        assert_eq!(false, result.unwrap());
    }

    #[test]
    fn test_evaluate_params() {
        let map = HashMap::from([("distance".to_string(), Evalue::Number(10))]);
        let result = Evaluator::new("distance > 5", &map).evaluate();
        assert_eq!(true, result.unwrap());
        let result = Evaluator::new("distance < 5", &map).evaluate();
        assert_eq!(false, result.unwrap());
        let result = Evaluator::new("distance = 10", &map).evaluate();
        assert_eq!(true, result.unwrap());
    }
}
