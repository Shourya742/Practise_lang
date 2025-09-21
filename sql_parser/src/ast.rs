use std::fmt;

use crate::expr::Expr;

#[derive(Debug, Clone)]
pub enum Statement<'a> {
    Query(Query<'a>),
}

#[derive(Debug, Clone)]
pub enum Query<'a> {
    Select(Box<SelectStmt<'a>>),
    With {
        with: With<'a>,
        query: Box<Query<'a>>,
    },
    Union {
        left: Box<Query<'a>>,
        all: bool,
        right: Box<Query<'a>>,
    },
}

#[derive(Debug, Clone)]
pub struct SelectStmt<'a> {
    pub projection: Vec<Expr<'a>>,
    pub from: Option<TableRef<'a>>,
    pub where_clause: Option<Expr<'a>>,
}

#[derive(Debug, Clone)]
pub struct TableRef<'a> {
    pub name: &'a str,
    pub alias: Option<&'a str>,
}

#[derive(Debug, Clone)]
pub struct With<'a> {
    pub recursive: bool,
    pub ctes: Vec<CTE<'a>>,
}

#[derive(Debug, Clone)]
#[allow(clippy::upper_case_acronyms)]
pub struct CTE<'a> {
    pub name: &'a str,
    pub columns: Option<Vec<&'a str>>,
    pub query: Box<Query<'a>>,
}

impl<'a> fmt::Display for Statement<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Query(q) => write!(f, "{}", q),
        }
    }
}

impl<'a> fmt::Display for Query<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Query::Select(s) => write!(f, "{}", s),
            Query::With { with, query } => {
                write!(f, "{} {}", with, query)
            }
            Query::Union { left, all, right } => {
                write!(
                    f,
                    "{} UNION {} {}",
                    left,
                    if *all { "ALL" } else { "" },
                    right
                )
            }
        }
    }
}

impl<'a> fmt::Display for SelectStmt<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SELECT ")?;

        for (i, expr) in self.projection.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", expr)?;
        }

        if let Some(from) = &self.from {
            write!(f, " FROM {}", from)?;
        }

        if let Some(where_clause) = &self.where_clause {
            write!(f, " WHERE {}", where_clause)?;
        }

        Ok(())
    }
}

impl<'a> fmt::Display for TableRef<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)?;
        if let Some(alias) = self.alias {
            write!(f, " AS {}", alias)?;
        }
        Ok(())
    }
}

impl<'a> fmt::Display for With<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "WITH ")?;
        if self.recursive {
            write!(f, "RECURSIVE ")?;
        }

        for (i, cte) in self.ctes.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", cte)?;
        }

        Ok(())
    }
}

impl<'a> fmt::Display for CTE<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)?;

        if let Some(columns) = &self.columns {
            write!(f, "(")?;
            for (i, col) in columns.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", col)?;
            }
            write!(f, ")")?;
        }

        write!(f, " AS ({})", self.query)
    }
}



#[cfg(test)]
mod tests {
    use crate::parser::parse_sql_to_string;

    #[test]
    fn test_simple_cte() {
        let sql = "WITH t AS (SELECT 1) SELECT * FROM t";
        let result = parse_sql_to_string(sql);
        assert!(result.is_ok());
        let ast_str = result.unwrap();
        assert!(ast_str.contains("With"));
        assert!(ast_str.contains("ctes"));
    }

    #[test]
    fn test_recursive_cte() {
        let sql = "WITH RECURSIVE t(n) AS (SELECT 1) SELECT * FROM t";
        let result = parse_sql_to_string(sql);
        assert!(result.is_ok());
        let ast_str = result.unwrap();
        assert!(ast_str.contains("recursive: true"));
    }
}