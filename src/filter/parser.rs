use super::lexer::{Lexer, TokenKind};

trait Expr {}

struct BinaryOp {
    left: Box<dyn Expr>,
    operand: TokenKind,
    right: Box<dyn Expr>,
}

struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl Parser<'_> {
    pub fn new<'a>(expr: &'a str) -> Parser<'a> {
        let lexer = Lexer::new(expr);
        Parser { lexer }
    }

    pub fn parse(&mut self) -> Box<dyn Expr> {
        let token = self.lexer.next();

        match token.kind {
            _ => panic!("unexpected token: {:?}", token),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_expression() {
        Parser::new("distance > 10m").parse();
    }
}
