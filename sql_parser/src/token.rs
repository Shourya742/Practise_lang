use std::{fmt::Display, ops::Range};

use logos::Logos;


#[derive(Logos, Debug, Clone, Copy, PartialEq, Eq)]
#[logos(skip r"[ \t\r\n\f]+")] // Skip whitespace
#[logos(skip r"--[^\n]*")] // Skip SQL comments 
pub enum TokenKind {
    // Keywords (case-insensitive)
    #[regex("(?i)SELECT")]
    Select,
    #[regex("(?i)FROM")]
    From,

    #[regex("(?i)WHERE")]
    Where,

    #[regex("(?i)WITH")]
    With,

    #[regex("(?i)RECURSIVE")]
    Recursive,

    #[regex("(?i)AS")]
    As,

    #[regex("(?i)UNION")]
    Union,

    #[regex("(?i)ALL")]
    All,

    #[regex("(?i)AND")]
    And,

    #[regex("(?i)OR")]
    Or,

    #[regex("(?i)INSERT")]
    Insert,

    #[regex("(?i)UPDATE")]
    Update,

    #[regex("(?i)DELETE")]
    Delete,

    // Identifiers and literals
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,

    #[regex(r"'([^'\\]|\\.)*'")]
    String,

    #[regex(r"-?[0-9]+")]
    Number,

    #[regex(r"-?[0-9]+\.[0-9]+")]
    Float,

    // Operators
    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Star,

    #[token("/")]
    Slash,

    #[token("=")]
    Equal,

    #[token("!=")]
    #[token("<>")]
    NotEqual,

    #[token("<")]
    Less,

    #[token(">")]
    Greater,

    #[token("<=")]
    LessEqual,

    #[token(">=")]
    GreaterEqual,

    // Delimiters
    #[token("(")]
    LeftParen,

    #[token(")")]
    RightParen,

    #[token(",")]
    Comma,

    #[token(";")]
    Semicolon,

    // End of input
    Eof,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token<'a> {
    pub text: &'a str,
    pub kind: TokenKind,
    pub span: Range<usize>
}


impl<'a> Token<'a> {
    pub fn new(text: &'a str, kind: TokenKind, span: Range<usize>) -> Self {
        Token { text, kind, span }
    }
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}({})", self.kind, self.text)
    }
}


pub fn tokenize(input: &str) -> Vec<Token<'_>> {
    let mut tokens = Vec::new();
    let mut lexer = TokenKind::lexer(input);

    while let Some(result) = lexer.next() {
        if let Ok(kind) = result {
            let span = lexer.span();
            let text = &input[span.clone()];
            tokens.push(Token::new(text, kind, span));
        }
    }

    let len = input.len();

    tokens.push(Token::new("", TokenKind::Eof, len..len));
    tokens
}


pub fn show_memory_usage(sql: &str) {
    println!("\n=== Zero-Copy Memory Demo ===");
    println!("Input SQL: {} bytes", sql.len());
    let tokens = tokenize(sql);
    let metadata_size = tokens.len() * std::mem::size_of::<Token>();
    println!("Token count: {}", tokens.len());
    println!("Token metadata size: {} bytes", metadata_size);
    println!("\nToken text pointers:");

    for (i, token) in tokens.iter().take(3).enumerate() {
        let ptr = token.text.as_ptr() as usize;
        let input_ptr = sql.as_ptr() as usize;
        if ptr >= input_ptr && ptr < input_ptr + sql.len() {
            println!("  Token {}: Points into original input ✓", i);
        }
    }
    println!("\n💡 Key insight: All token strings are slices of the original input!");
    println!("   No string copying = massive memory savings");
}

#[cfg(test)]
mod tests {
    use crate::token::{tokenize, TokenKind};

    #[test]
    fn test_tokenize_basic() {
        let sql = "SELECT name FROM users WHERE age > 18";
        let tokens = tokenize(sql);

        assert_eq!(tokens[0].kind, TokenKind::Select);
        assert_eq!(tokens[1].kind, TokenKind::Identifier);
        assert_eq!(tokens[1].text, "name");
        assert_eq!(tokens[2].kind, TokenKind::From);
    }
}