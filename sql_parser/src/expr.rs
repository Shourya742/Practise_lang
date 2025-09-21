use crate::token::TokenKind;




#[derive(Debug, Clone, PartialEq)]
pub enum Literal<'a> {
    Number(i64),
    Float(f64),
    String(&'a str)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    And,
    Or,
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Plus,
    Minus,
    Multiply,
    Divide
}

pub enum Expr<'a> {
    Column(&'a str),
    Literal(Literal<'a>),
    Binary {
        left: Box<Expr<'a>>,
        op: BinaryOp,
        right: Box<Expr<'a>>
    },
    Paren(Box<Expr<'a>>),
    Star
}

fn get_precedence(token: TokenKind) -> Option<(u8, bool)> {
    match token {
        TokenKind::Or => Some((10, true)),
        TokenKind::And => Some((20, true)),
        TokenKind::Equal | TokenKind::NotEqual => Some((30, true)),
        TokenKind::Less | TokenKind::Greater | TokenKind::LessEqual | TokenKind::GreaterEqual => {
            Some((40, true))
        }
        TokenKind::Plus | TokenKind::Minus => Some((50, true)),
        TokenKind::Star | TokenKind::Slash => Some((60, true)),
        _ => None,
    }
}


impl<'a> std::fmt::Display for Expr<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Column(name) => write!(f, "{}", name),
            Expr::Literal(lit) => write!(f, "{}", lit),
            Expr::Binary { left, op, right } => {
                write!(f, "({} {} {})", left, op, right)
            }
            Expr::Paren(expr) => write!(f, "({})", expr),
            Expr::Star => write!(f, "*"),
        }
    }
}

impl<'a> std::fmt::Display for Literal<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Number(n) => write!(f, "{}", n),
            Literal::Float(fl) => write!(f, "{}", fl),
            Literal::String(s) => write!(f, "'{}'", s),
        }
    }
}

impl std::fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            BinaryOp::And => "AND",
            BinaryOp::Or => "OR",
            BinaryOp::Equal => "=",
            BinaryOp::NotEqual => "!=",
            BinaryOp::Less => "<",
            BinaryOp::Greater => ">",
            BinaryOp::LessEqual => "<=",
            BinaryOp::GreaterEqual => ">=",
            BinaryOp::Plus => "+",
            BinaryOp::Minus => "-",
            BinaryOp::Multiply => "*",
            BinaryOp::Divide => "/",
        };
        write!(f, "{}", s)
    }
}
