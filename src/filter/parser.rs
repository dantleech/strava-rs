use super::lexer::{Lexer, TokenKind};

#[derive(PartialEq, Eq, Debug)]
pub enum Expr {
    Binary(Box<Expr>, TokenKind, Box<Expr>),
    Number(u16),
    Variable(String),
}

struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl Parser<'_> {
    pub fn new<'a>(expr: &'a str) -> Parser<'a> {
        let lexer = Lexer::new(expr);
        Parser { lexer }
    }

    pub fn parse(&mut self) -> Result<Expr, &'static str> {
        let token = self.lexer.next();
        let left: Result<Expr, &str> = match token.kind {
            TokenKind::Number => match self.lexer.token_value(token).parse::<u16>() {
                Ok(v) => Ok(Expr::Number(v)),
                Err(_) => Err("Could not number"),
            },
            TokenKind::Name => {
                let value = self.lexer.token_value(token);
                Ok(Expr::Variable(value.to_string()))
            }
            _ => Err("foo"),
        };

        left
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_expression() {
        assert_eq!(Expr::Variable("distance".to_string()), Parser::new("distance").parse().unwrap());
        assert_eq!(Expr::Number(10), Parser::new("10").parse().unwrap());
    }
}
