use super::parser::Parser;

struct Evaluator<'a, TSubject> {
    parser: Parser<'a>,
    subject: TSubject,
}

#[derive(PartialEq, Eq, PartialOrd, Debug)]
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

impl<T> Evaluator<'_, T> {
    pub fn new<'a, TSubject>(expr: &'a str, subject: TSubject) -> Evaluator<TSubject> {
        Evaluator::<TSubject> {
            parser: Parser::new(expr),
            subject,
        }
    }

    pub fn evaluate(&mut self) -> Result<bool, String> {
        let expr = self.parser.parse()?;
        match self.evaluate_expr(expr.clone())? {
            Evalue::String(_)|
            Evalue::Number(_) => Err(
                format!(
                    "expression must evluate to a boolean, got: {:?}",
                    expr
                ).to_string()
            ),
            Evalue::Bool(b) => Ok(b),
        }
    }

    fn evaluate_expr<>(&self, expr: super::parser::Expr) -> Result<Evalue, String> {
        match expr {
            super::parser::Expr::Boolean(b) => Ok(Evalue::Bool(b)),
            super::parser::Expr::Binary(
                lexpr,
                op,
                rexpr,
            ) => {
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
                    _ => Err(format!("unknown operator: {:?}", op))
                }?;
                Ok(Evalue::Bool(eval))
            },
            super::parser::Expr::Number(n) => Ok(Evalue::Number(n)),
            super::parser::Expr::Variable(_) => Ok(Evalue::Number(1)),
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
        let result = Evaluator::<TestSubject>::new(
            "false",
            TestSubject { distance: 100 },
        ).evaluate();
        assert_eq!(false, result.unwrap());
        let result = Evaluator::<TestSubject>::new(
            "20 > 10",
            TestSubject { distance: 100 },
        ).evaluate();

        assert_eq!(true, result.unwrap());

        let result = Evaluator::<TestSubject>::new(
            "20 > 10 and false",
            TestSubject { distance: 100 },
        ).evaluate();

        assert_eq!(false, result.unwrap());
    }
}
