use crate::{
    error::{Backtrace, ParseError},
    token::{Token, TokenKind, tokenize},
};

pub type ParseResult<T> = Result<T, ParseError>;

pub struct Parser<'a> {
    tokens: &'a [Token<'a>],
    pos: usize,
    backtrace: &'a Backtrace,
    input: &'a str,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token<'a>], backtrace: &'a Backtrace, input: &'a str) -> Self {
        Parser {
            tokens,
            pos: 0,
            backtrace,
            input,
        }
    }

    pub fn current(&self) -> Option<&Token<'a>> {
        self.tokens.get(self.pos)
    }

    pub fn advance(&mut self) -> &Token<'a> {
        let token = &self.tokens[self.pos];
        if self.pos < self.tokens.len() - 1 {
            self.pos += 1;
        }
        token
    }

    pub fn expect(&mut self, expected: TokenKind) -> ParseResult<&Token<'a>> {
        match self.current() {
            Some(token) if token.kind == expected => Ok(self.advance()),
            Some(token) => {
                self.backtrace.track_error(
                    token.span.start,
                    &format!("{expected:?}"),
                    Some(token.text),
                    self.input,
                );
                Err(self.backtrace.get_error(self.input))
            }
            None => {
                let pos = if self.pos > 0 && !self.tokens.is_empty() {
                    self.tokens[self.pos - 1].span.end
                } else {
                    0
                };
                self.backtrace
                    .track_error(pos, &format!("{:?}", expected), None, self.input);
                Err(self.backtrace.get_error(self.input))
            }
        }
    }

    pub fn try_consume(&mut self, kind: TokenKind) -> bool {
        if self.current().map(|t| t.kind) == Some(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    pub fn parse_identifier(&mut self) -> ParseResult<&'a str> {
        match self.current() {
            Some(token) if token.kind == TokenKind::Identifier => Ok(self.advance().text),
            Some(token) => {
                self.backtrace.track_error(
                    token.span.start,
                    "identifier",
                    Some(token.text),
                    self.input,
                );
                Err(self.backtrace.get_error(self.input))
            }
            None => {
                let pos = if self.pos > 0 && !self.tokens.is_empty() {
                    self.tokens[self.pos - 1].span.end
                } else {
                    0
                };
                self.backtrace
                    .track_error(pos, "identifier", None, self.input);
                Err(self.backtrace.get_error(self.input))
            }
        }
    }

    pub fn error_at_current(&self, msg: &str) -> ParseError {
        let mut error = self.backtrace.get_error(self.input);
        error.message = msg.to_string();
        error
    }

    /// Check if current token might be a typo for the expected keyword
    fn check_for_keyword_typo(
        &mut self,
        expected_keyword: &str,
        starts_with_chars: &[char],
    ) -> bool {
        if let Some(token) = self.current() {
            if token.kind == TokenKind::Identifier {
                let text_upper = token.text.to_uppercase();
                for &ch in starts_with_chars {
                    if text_upper.starts_with(ch) {
                        self.backtrace.track_error(
                            token.span.start,
                            expected_keyword,
                            Some(token.text),
                            self.input,
                        );
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Check if current token might be a typo for WHERE keyword (with substring check)
    fn check_for_where_typo(&mut self) -> bool {
        if let Some(token) = self.current() {
            if token.kind == TokenKind::Identifier {
                let text_upper = token.text.to_uppercase();
                if text_upper.starts_with('W') || text_upper.contains("HER") {
                    self.backtrace.track_error(
                        token.span.start,
                        "WHERE",
                        Some(token.text),
                        self.input,
                    );
                    return true;
                }
            }
        }
        false
    }

    /// Check if current token is a specific WHERE typo pattern
    fn check_for_specific_where_typos(&mut self) -> bool {
        if let Some(token) = self.current() {
            if token.kind == TokenKind::Identifier {
                let text = token.text.to_uppercase();
                if text.starts_with("WHEER")
                    || text.starts_with("WHER")
                    || text.starts_with("WHRE")
                    || text == "WHEER"
                {
                    self.backtrace.track_error(
                        token.span.start,
                        "WHERE",
                        Some(token.text),
                        self.input,
                    );
                    return true;
                }
            }
        }
        false
    }
}

pub fn parse_sql(sql: &str) -> Result<(), ParseError> {
    let tokens = tokenize(sql);
    let backtrace = Backtrace::new();
    let mut parser = Parser::new(&tokens, &backtrace, sql);
    Ok(())
}
