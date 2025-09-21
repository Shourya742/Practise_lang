use crate::{
    ast::{CTE, Query, SelectStmt, Statement, TableRef, With},
    error::{Backtrace, ParseError},
    expr::Expr,
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

    pub fn parse_query(&mut self) -> ParseResult<Query<'a>> {
        if self.current().map(|t| t.kind) == Some(TokenKind::With) {
            let with = self.parse_with()?;
            let query = Box::new(self.parse_query()?);
            Ok(Query::With { with, query })
        } else {
            let select = self.parse_select()?;

            if self.try_consume(TokenKind::Union) {
                let all = self.try_consume(TokenKind::All);
                let right = Box::new(self.parse_query()?);
                Ok(Query::Union {
                    left: Box::new(Query::Select(Box::new(select))),
                    all,
                    right,
                })
            } else {
                Ok(Query::Select(Box::new(select)))
            }
        }
    }

    fn parse_identifier_lis(&mut self) -> ParseResult<Vec<&'a str>> {
        let mut idents = vec![self.parse_identifier()?];

        while self.try_consume(TokenKind::Comma) {
            idents.push(self.parse_identifier()?);
        }
        Ok(idents)
    }

    fn parse_cte(&mut self) -> ParseResult<CTE<'a>> {
        let name = self.parse_identifier()?;
        let columns = if self.current().map(|t| t.kind) == Some(TokenKind::LeftParen) {
            self.advance();
            let cols = self.parse_identifier_lis()?;
            self.expect(TokenKind::RightParen)?;
            Some(cols)
        } else {
            None
        };

        self.expect(TokenKind::As)?;
        self.expect(TokenKind::LeftParen)?;
        let query = Box::new(self.parse_query()?);
        self.expect(TokenKind::RightParen)?;
        Ok(CTE {
            name,
            columns,
            query,
        })
    }

    pub fn parse_with(&mut self) -> ParseResult<With<'a>> {
        self.expect(TokenKind::With)?;

        let recursive = self.try_consume(TokenKind::Recursive);

        let mut ctes = vec![self.parse_cte()?];

        while self.try_consume(TokenKind::Comma) {
            ctes.push(self.parse_cte()?);
        }
        Ok(With { recursive, ctes })
    }

    fn parse_expr_list(&mut self) -> ParseResult<Vec<Expr<'a>>> {
        let mut exprs = vec![self.parse_expr()?];

        while self.try_consume(TokenKind::Comma) {
            exprs.push(self.parse_expr()?);
        }
        Ok(exprs)
    }

    fn parse_table_ref(&mut self) -> ParseResult<TableRef<'a>> {
        let name = self.parse_identifier()?;

        let alias = if self.try_consume(TokenKind::As) {
            Some(self.parse_identifier()?)
        } else if let Some(token) = self.current() {
            if token.kind == TokenKind::Identifier {
                if self.check_for_specific_where_typos() {
                    return Err(self.backtrace.get_error(self.input));
                }
                Some(self.parse_identifier()?)
            } else {
                None
            }
        } else {
            None
        };

        Ok(TableRef { name, alias })
    }

    pub fn parse_select(&mut self) -> ParseResult<SelectStmt<'a>> {
        let mut had_errors = false;

        match self.current() {
            Some(token) if token.kind == TokenKind::Select => {
                self.advance();
            }
            Some(token) if token.kind == TokenKind::Identifier => {
                self.backtrace.track_error(
                    token.span.start,
                    "SELECT",
                    Some(token.text),
                    self.input,
                );
                had_errors = true;

                let text = token.text.to_uppercase();
                if text.starts_with("SEL") && text.len() >= 4 {
                    self.advance();
                } else {
                    return Err(self.backtrace.get_error(self.input));
                }
            }
            Some(token) => {
                self.backtrace.track_error(
                    token.span.start,
                    "SELECT",
                    Some(token.text),
                    self.input,
                );
                return Err(self.backtrace.get_error(self.input));
            }
            None => {
                let pos = if self.pos > 0 && !self.tokens.is_empty() {
                    self.tokens[self.pos - 1].span.end
                } else {
                    0
                };
                self.backtrace.track_error(pos, "SELECT", None, self.input);
                return Err(self.backtrace.get_error(self.input));
            }
        }

        let projection = if self.try_consume(TokenKind::Star) {
            vec![Expr::Star]
        } else {
            self.parse_expr_list()?
        };

        let from = if self.try_consume(TokenKind::From) {
            Some(self.parse_table_ref()?)
        } else {
            if self.check_for_keyword_typo("FROM", &['F']) {
                return Err(self.backtrace.get_error(self.input));
            }
            None
        };

        let where_clause = if self.try_consume(TokenKind::Where) {
            Some(self.parse_expr()?)
        } else {
            if self.check_for_where_typo() {
                return Err(self.backtrace.get_error(self.input));
            }
            None
        };

        if had_errors {
            return Err(self.backtrace.get_error(self.input));
        }

        Ok(SelectStmt {
            projection,
            from,
            where_clause,
        })
    }

    pub fn parse_statement(&mut self) -> ParseResult<Statement<'a>> {
        let start_pos = self.pos;

        if self.current().map(|t| t.kind) == Some(TokenKind::With) {
            match self.parse_with() {
                Ok(with) => match self.parse_query() {
                    Ok(query) => {
                        return Ok(Statement::Query(Query::With {
                            with,
                            query: Box::new(query),
                        }));
                    }
                    Err(_) => {
                        self.pos = start_pos;
                    }
                },
                Err(_) => {
                    self.pos = start_pos;
                }
            }
        }

        self.pos = start_pos;
        match self.parse_select() {
            Ok(stmt) => return Ok(Statement::Query(Query::Select(Box::new(stmt)))),
            Err(_) => {}
        }
        self.pos = start_pos;
        if let Some(token) = self.current() {
            self.backtrace
                .track_error(token.span.start, "INSERT", Some(token.text), self.input);
            self.backtrace
                .track_error(token.span.start, "UPDATE", Some(token.text), self.input);
            self.backtrace
                .track_error(token.span.start, "DELETE", Some(token.text), self.input);
            self.backtrace
                .track_error(token.span.start, "WITH", Some(token.text), self.input);
        }

        Err(self.backtrace.get_error(self.input))
    }
}

pub fn parse_sql(sql: &str) -> Result<(), ParseError> {
    let tokens = tokenize(sql);
    let backtrace = Backtrace::new();
    let mut parser = Parser::new(&tokens, &backtrace, sql);
    let _stmt = parser.parse_statement()?;
    Ok(())
}

pub fn parse_sql_to_string(sql: &str) -> Result<String, ParseError> {
    use crate::token::tokenize;

    let tokens = tokenize(sql);
    let backtrace = Backtrace::new();
    let mut parser = Parser::new(&tokens, &backtrace, sql);

    let stmt = parser.parse_statement()?;
    Ok(format!("{:?}", stmt))
}
