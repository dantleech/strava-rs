use super::lexer::{Lexer, Token, TokenKind};

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Expr {
    Binary(Box<Expr>, TokenKind, Box<Expr>),
    Number(u16),
    Variable(String),
    Quantity(Box<Expr>, QuantityUnit)
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum QuantityUnit {
    Miles,
    Kilometers,
}

struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl Parser<'_> {
    pub fn new<'a>(expr: &'a str) -> Parser<'a> {
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
            TokenKind::Number => match self.lexer.token_value(&token).parse::<u16>() {
                Ok(v) => Ok(Expr::Number(v)),
                Err(_) => Err("Could not parse number".to_string()),
            },
            TokenKind::Name => {
                let value = self.lexer.token_value(&token);
                Ok(Expr::Variable(value.to_string()))
            }
            _ => Err(format!("unknown left token: {:?} at {}", token.kind, token.start)),
        }?;

        let mut next_t = self.lexer.next();
        if next_t.kind == TokenKind::Eol {
            return Ok((left, next_t));
        }

        // suffix parsing
        let suffix = match &next_t.kind {
            TokenKind::Name => match self.lexer.token_value(&next_t) {
                "m" => Some(Expr::Quantity(Box::new(left.clone()), QuantityUnit::Miles)),
                "km" => Some(Expr::Quantity(Box::new(left.clone()), QuantityUnit::Kilometers)),
                _ => None,
            },
            _ => None,
        };
        (next_t, left) = match suffix {
            Some(suffix) => {
                (self.lexer.next(), suffix)
            },
            None => (next_t, left),
        };

        // infix parsing
        while precedence < self.token_precedence(&next_t) {
            let (right, new_t) = self.parse_expr(self.token_precedence(&next_t)).unwrap();
            left = match &next_t.kind {
                TokenKind::GreaterThan
                | TokenKind::GreaterThanEqual
                | TokenKind::And
                | TokenKind::Or
                | TokenKind::Equal
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
            TokenKind::Number => 100,
            TokenKind::Unkown => 100,
            TokenKind::Colon => 100,
            TokenKind::Name => 100,
            TokenKind::Eol => 0,
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
        assert_eq!(Expr::Number(10), Parser::new("10").parse().unwrap());
        assert_eq!(
            Expr::Binary(
                Box::new(Expr::Number(10)),
                TokenKind::GreaterThan,
                Box::new(Expr::Number(20))
            ),
            Parser::new("10 > 20").parse().unwrap()
        );
        assert_eq!(
            Expr::Binary(
                Box::new(Expr::Binary(
                    Box::new(Expr::Variable("variable".to_string())),
                    TokenKind::GreaterThan,
                    Box::new(Expr::Number(20))
                )),
                TokenKind::And,
                Box::new(Expr::Binary(
                    Box::new(Expr::Number(10)),
                    TokenKind::LessThan,
                    Box::new(Expr::Number(30))
                )),
            ),
            Parser::new("variable > 20 and 10 < 30").parse().unwrap()
        );
        assert_eq!(
            Expr::Quantity(Box::new(Expr::Number(10)), QuantityUnit::Miles),
            Parser::new("10m").parse().unwrap()
        );
    }
}
