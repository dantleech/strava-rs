// distance > 10 AND distance < 20
// type = Run
// pace > 06:00
// average_speed > 10mph

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum TokenKind {
    Number,
    Contains,
    Unkown,
    Colon,
    GreaterThan,
    GreaterThanEqual,
    LessThanEqual,
    LessThan,
    Or,
    And,
    Equal,
    Name,
    Eol,
}

fn is_number(c: char) -> bool {
    match c {
        '0'..='9' => true,
        _ => false,
    }
}
fn is_name(c: char) -> bool {
    match c {
        'a'..='z' => true,
        'A'..='Z' => true,
        _ => false,
    }
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub start: usize,
    pub length: usize,
}

pub struct Lexer<'a> {
    pub pos: usize,
    pub expr: &'a str,
}

impl Lexer<'_> {
    pub fn new<'a>(expr: &'a str) -> Lexer<'_> {
        Lexer { expr, pos: 0 }
    }
    pub fn next(&mut self) -> Token {
        self.skip_whitespace();
        let c = self.current();
        let t = match c {
            '\0' => self.spawn_token(TokenKind::Eol, self.pos),
            _ => {
                if is_number(c) {
                    return self.parse_number();
                }

                if is_name(c) {
                    return self.parse_name();
                }

                match c {
                    ':' => self.spawn_advance(TokenKind::Colon, 1),
                    '>' => match self.peek(1) {
                        '=' => self.spawn_advance(TokenKind::GreaterThanEqual, 2),
                        _ => self.spawn_advance(TokenKind::GreaterThan, 1),
                    },
                    '<' => match self.peek(1) {
                        '=' => self.spawn_advance(TokenKind::LessThanEqual, 2),
                        _ => self.spawn_advance(TokenKind::LessThan, 1),
                    },
                    _ => self.spawn_advance(TokenKind::Unkown, 0),
                }
            }
        };
        t
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn current(&self) -> char {
        match self.expr.chars().nth(self.pos) {
            Some(s) => s,
            None => '\0',
        }
    }

    fn peek(&self, amount: usize) -> char {
        match self.expr.chars().nth(self.pos + amount) {
            Some(s) => s,
            None => '\0',
        }
    }

    fn parse_number(&mut self) -> Token {
        let start = self.pos;
        while is_number(self.current()) {
            self.advance()
        }

        self.spawn_token(TokenKind::Number, start)
    }

    fn parse_name(&mut self) -> Token {
        let mut length = 0;
        while is_name(self.peek(length)) {
            length += 1;
        }

        match &self.expr[self.pos..self.pos + length] {
            "or" => self.spawn_advance(TokenKind::Or, length),
            "and" => self.spawn_advance(TokenKind::And, length),
            "OR" => self.spawn_advance(TokenKind::Or, length),
            "AND" => self.spawn_advance(TokenKind::And, length),
            _ => self.spawn_advance(TokenKind::Name, length),
        }
    }

    fn spawn_token(&self, number: TokenKind, start: usize) -> Token {
        Token {
            kind: number,
            start,
            length: self.pos - start,
        }
    }

    fn skip_whitespace(&mut self) {
        while ' ' == self.current() {
            self.advance();
        }
    }

    pub fn token_value(&self, token: &Token) -> &str {
        &self.expr[token.start..token.start + token.length]
    }

    fn spawn_advance(&mut self, kind: TokenKind, length: usize) -> Token {
        let t = Token {
            kind,
            start: self.pos,
            length,
        };
        self.pos += length;
        return t;
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn lex_int() {
        assert_eq!(TokenKind::Number, Lexer::new("10").next().kind);
        assert_eq!(2, Lexer::new("10").next().length);
        assert_eq!(0, Lexer::new("10").next().start);
    }

    #[test]
    pub fn lex_skip_whitespace() {
        let mut l = Lexer::new("    10");
        let t = l.next();
        assert_eq!(TokenKind::Number, t.kind);
        assert_eq!("10", l.token_value(&t))
    }

    #[test]
    pub fn lex_eof() {
        let mut l = Lexer::new("    10");
        assert_eq!(TokenKind::Number, l.next().kind);
        assert_eq!(TokenKind::Eol, l.next().kind);
    }
    #[test]
    pub fn lex_symbols() {
        let mut l = Lexer::new(" :");
        assert_eq!(TokenKind::Colon, l.next().kind);
    }

    #[test]
    pub fn lex_comparators() {
        assert_eq!(TokenKind::GreaterThanEqual, Lexer::new(">=").next().kind);
        assert_eq!(TokenKind::GreaterThan, Lexer::new(">").next().kind);
        assert_eq!(TokenKind::LessThanEqual, Lexer::new("<=").next().kind);
        assert_eq!(TokenKind::LessThan, Lexer::new("<").next().kind);
    }

    #[test]
    pub fn lex_logical_operators() {
        assert_eq!(TokenKind::Or, Lexer::new("or").next().kind);
        assert_eq!(TokenKind::And, Lexer::new("and").next().kind);
        assert_eq!(TokenKind::Or, Lexer::new("OR").next().kind);
        assert_eq!(TokenKind::And, Lexer::new("AND").next().kind);
    }

    #[test]
    pub fn lex_expression() {
        let mut l = Lexer::new("distance > 10m");
        let t = l.next();
        assert_eq!(TokenKind::Name, t.kind);
        assert_eq!("distance", l.token_value(&t));
        let t = l.next();
        assert_eq!(TokenKind::GreaterThan, t.kind);
        assert_eq!(">", l.token_value(&t));
        let t = l.next();
        assert_eq!(TokenKind::Number, t.kind);
        assert_eq!("10", l.token_value(&t));
        let t = l.next();
        assert_eq!(TokenKind::Name, t.kind);
        assert_eq!("m", l.token_value(&t));
    }
}
