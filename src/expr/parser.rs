use std::str::FromStr;

use chrono::NaiveDate;

use super::lexer::{Lexer, Token, TokenKind};

#[derive(PartialEq, Debug, Clone)]
pub enum Expr {
    Binary(Box<Expr>, TokenKind, Box<Expr>),
    Number(f64),
    Variable(String),
    Boolean(bool),
    String(String),
    Date(NaiveDate),
    Quantity(Box<Expr>, QuantityUnit),
}

#[derive(PartialEq, Debug, Clone)]
pub enum QuantityUnit {
    Kmph,
    Mph,
}
impl QuantityUnit {
    pub(crate) fn convert(&self, val: f64) -> f64 {
        match self {
            QuantityUnit::Kmph => val * 1000.0,
            QuantityUnit::Mph => (val * 1.609344) * 1000.0,
        }
    }
}

impl From<&str> for QuantityUnit {
    fn from(value: &str) -> Self {
        match value {
            "mph" => Self::Mph,
            "kmph" => Self::Kmph,
            "m" => Self::Mph,
            "mi" => Self::Mph,
            "km" => Self::Kmph,
            "k" => Self::Kmph,
            _ => Self::Kmph,
        }
    }
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl Parser<'_> {
    pub fn new(expr: &str) -> Parser<'_> {
        let lexer = Lexer::new(expr);
        Parser { lexer }
    }

    pub fn parse(&mut self) -> Result<Expr, String> {
        match self.parse_expr(0) {
            Ok((expr, _)) => Ok(expr),
            Err(e) => Err(e),
        }
    }

    fn parse_expr(&mut self, precedence: usize) -> Result<(Expr, Token), String> {
        let token = self.lexer.next();
        let mut left: Expr = match token.kind {
            TokenKind::True => Ok(Expr::Boolean(true)),
            TokenKind::False => Ok(Expr::Boolean(false)),
            TokenKind::Number => match self.lexer.token_value(&token).parse::<f64>() {
                Ok(v) => Ok(Expr::Number(v)),
                Err(_) => Err("Could not parse number".to_string()),
            },
            TokenKind::String => Ok(Expr::String(self.lexer.token_value(&token).to_string())),
            TokenKind::Name => {
                let value = self.lexer.token_value(&token);
                Ok(Expr::Variable(value.to_string()))
            }
            TokenKind::Date => match NaiveDate::from_str(self.lexer.token_value(&token)) {
                Ok(d) => Ok(Expr::Date(d)),
                Err(_) => Err("Could not parse date".to_string()),
            },
            _ => Err(format!(
                "unknown left token: {:?} at {}",
                token.kind, token.start
            )),
        }?;

        let mut next_t = self.lexer.next();
        if next_t.kind == TokenKind::Eol {
            return Ok((left, next_t));
        }

        // suffix
        if next_t.kind == TokenKind::Name {
            left = Expr::Quantity(Box::new(left), QuantityUnit::from(self.lexer.token_value(&next_t)));
            next_t = self.lexer.next();
            if next_t.kind == TokenKind::Eol {
                return Ok((left, next_t));
            }
        }


        // infix parsing
        while precedence < self.token_precedence(&next_t) {
            let (right, new_t) = self.parse_expr(self.token_precedence(&next_t))?;
            left = match &next_t.kind {
                TokenKind::GreaterThan
                | TokenKind::GreaterThanEqual
                | TokenKind::And
                | TokenKind::Or
                | TokenKind::FuzzyEqual
                | TokenKind::Equal
                | TokenKind::NotFuzzyEqual
                | TokenKind::NotEqual
                | TokenKind::LessThanEqual
                | TokenKind::LessThan => Ok(Expr::Binary(
                    Box::new(left),
                    next_t.kind.clone(),
                    Box::new(right),
                )),
                _ => Err(format!(
                    "unknown infix token: {:?} at {}",
                    &next_t.kind, &next_t.start
                )),
            }?;
            next_t = new_t;
        }

        Ok((left, next_t))
    }

    fn token_precedence(&self, token: &super::lexer::Token) -> usize {
        match token.kind {
            TokenKind::Or => 10,
            TokenKind::And => 10,
            TokenKind::GreaterThan => 20,
            TokenKind::GreaterThanEqual => 20,
            TokenKind::LessThanEqual => 20,
            TokenKind::LessThan => 20,
            TokenKind::Equal => 20,
            TokenKind::Contains => 20,
            TokenKind::Eol => 0,
            _ => 100,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_expression() {
        assert_eq!(
            Expr::Variable("distance".to_string()),
            Parser::new("distance").parse().unwrap()
        );
        assert_eq!(Expr::Number(10.0), Parser::new("10").parse().unwrap());
        assert_eq!(Expr::Number(10.2), Parser::new("10.2").parse().unwrap());
        assert_eq!(
            Expr::Binary(
                Box::new(Expr::Number(10.0)),
                TokenKind::GreaterThan,
                Box::new(Expr::Number(20.0))
            ),
            Parser::new("10 > 20").parse().unwrap()
        );
        assert_eq!(
            Expr::Binary(
                Box::new(Expr::Binary(
                    Box::new(Expr::Variable("variable".to_string())),
                    TokenKind::GreaterThan,
                    Box::new(Expr::Number(20.0))
                )),
                TokenKind::And,
                Box::new(Expr::Binary(
                    Box::new(Expr::Number(10.0)),
                    TokenKind::LessThan,
                    Box::new(Expr::Number(30.0))
                )),
            ),
            Parser::new("variable > 20 and 10 < 30").parse().unwrap()
        );
        assert_eq!(Expr::Number(10.0), Parser::new("10").parse().unwrap());
        assert_eq!(
            Expr::Date(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()),
            Parser::new("2024-01-01").parse().unwrap(),
        );
    }

    #[test]
    fn parse_expression_quantity() {
        assert_eq!(Expr::Quantity(
            Box::new(Expr::Number(10.2)),
            QuantityUnit::Kmph
        ), Parser::new("10.2kmph").parse().unwrap());
    }
}
