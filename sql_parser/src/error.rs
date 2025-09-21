use std::{cell::RefCell, fmt::Display};

use colored::Colorize;
use strsim::jaro_winkler;

#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub suggestion: Option<String>,
    pub context: Option<String>,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{} at line {}:{}",
            "Parse error".red().bold(),
            self.line,
            self.column
        )?;

        writeln!(f, "  {}", self.message)?;

        if let Some(ref suggestion) = self.suggestion {
            writeln!(f, " {} {}", "Did you mean:".yellow(), suggestion.green())?;
        }

        if let Some(ref context) = self.context {
            writeln!(f, "\n{}", context)?;
        }

        Ok(())
    }
}

impl std::error::Error for ParseError {}

#[derive(Debug, Clone)]
struct BacktraceInner {
    furthest_pos: usize,
    expected: Vec<String>,
    found: Option<String>,
    line: usize,
    column: usize,
}

#[derive(Debug, Default)]
pub struct Backtrace {
    inner: RefCell<Option<BacktraceInner>>,
}

impl Backtrace {
    pub fn new() -> Self {
        Self::default()
    }

    /// Track an error if it's the furthest we've reached
    pub fn track_error(&self, pos: usize, expected: &str, found: Option<&str>, input: &str) {
        let (line, column) = position_to_line_col(input, pos);

        let mut inner = self.inner.borrow_mut();

        match &mut *inner {
            None => {
                *inner = Some(BacktraceInner {
                    furthest_pos: pos,
                    expected: vec![expected.to_string()],
                    found: found.map(|s| s.to_string()),
                    line,
                    column,
                })
            }
            Some(existing) => {
                if pos > existing.furthest_pos {
                    *existing = BacktraceInner {
                        furthest_pos: pos,
                        expected: vec![expected.to_string()],
                        found: found.map(|s| s.to_string()),
                        line,
                        column,
                    };
                } else if pos == existing.furthest_pos {
                    if !existing.expected.contains(&expected.to_string()) {
                        existing.expected.push(expected.to_string())
                    }
                }
            }
        }
    }

    pub fn get_error(&self, input: &str) -> ParseError {
        let inner = self.inner.borrow();
        match &*inner {
            None => ParseError {
                message: "Unexpected error".to_string(),
                line: 1,
                column: 1,
                suggestion: None,
                context: None,
            },
            Some(inner) => {
                let suggestion = inner
                    .found
                    .as_ref()
                    .and_then(|found| suggest_keyword(&found));
                let context = get_error_context(input, inner.furthest_pos);
                let expected_str = if inner.expected.len() == 1 {
                    inner.expected[0].clone()
                } else {
                    format!("one of: {}", inner.expected.join(", "))
                };

                let message = match &inner.found {
                    Some(found) => format!("Expected {}, found: `{}`", expected_str, found),
                    None => format!("Expected {}, reached out of input", expected_str),
                };

                ParseError {
                    message,
                    line: inner.line,
                    column: inner.column,
                    suggestion,
                    context,
                }
            }
        }
    }
}

fn suggest_keyword(input: &str) -> Option<String> {
    const KEYWORDS: &[&str] = &[
        "SELECT",
        "FROM",
        "WHERE",
        "WITH",
        "RECURSIVE",
        "INSERT",
        "UPDATE",
        "DELETE",
        "UNION",
        "ALL",
        "AND",
        "OR",
        "AS",
        "JOIN",
        "LEFT",
        "RIGHT",
        "INNER",
        "OUTER",
        "ON",
        "GROUP",
        "ORDER",
        "BY",
        "HAVING",
        "LIMIT",
        "OFFSET",
    ];

    let input_upper = input.to_uppercase();
    KEYWORDS
        .iter()
        .map(|&keyword| (keyword, jaro_winkler(&input_upper, keyword)))
        .filter(|(_, score)| *score > 0.8)
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .map(|(keyword, _)| keyword.to_string())
}

fn position_to_line_col(input: &str, pos: usize) -> (usize, usize) {
    let mut line = 1;
    let mut col = 1;

    for (i, ch) in input.chars().enumerate() {
        if i >= pos {
            break;
        }

        if ch == '\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
    }

    (line, col)
}

fn get_error_context(input: &str, pos: usize) -> Option<String> {
    let lines: Vec<&str> = input.lines().collect();
    let (line_num, col) = position_to_line_col(input, pos);

    if line_num > 0 && line_num <= lines.len() {
        let line = lines[line_num - 1];
        let mut result = String::new();

        result.push_str(&format!("  {} | {}\n", line_num, line));

        result.push_str(&format!("  {} | ", " ".repeat(line_num.to_string().len())));
        result.push_str(&" ".repeat(col - 1));
        result.push_str(&"^".red().to_string());

        Some(result)
    } else {
        None
    }
}

/// Demonstrate the error tracking system with realistic backtracking
pub fn demonstrate_error_tracking() {
    println!("\n=== Error Tracking with Backtracking ===\n");

    // Example 1: Simple typo at the start
    println!("{}", "Example 1: Typo at start".yellow());
    let input1 = "SELCT * FROM users";
    let backtrace1 = Backtrace::new();

    println!("Input: {}", input1.red());
    println!("\nParser tries different statement types:");
    println!("  Position 0: Try SELECT... failed");
    backtrace1.track_error(0, "SELECT", Some("SELCT"), input1);
    println!("  Position 0: Try INSERT... failed");
    backtrace1.track_error(0, "INSERT", Some("SELCT"), input1);
    println!("  Position 0: Try UPDATE... failed");
    backtrace1.track_error(0, "UPDATE", Some("SELCT"), input1);
    println!("  Position 0: Try DELETE... failed");
    backtrace1.track_error(0, "DELETE", Some("SELCT"), input1);
    println!("  Position 0: Try WITH... failed");
    backtrace1.track_error(0, "WITH", Some("SELCT"), input1);

    let error1 = backtrace1.get_error(input1);
    println!("\n{} {}", "Result:".green(), error1.message);
    if let Some(suggestion) = error1.suggestion {
        println!("  {} {}", "Suggestion:".cyan(), suggestion);
    }

    // Example 2: Error deeper in the query
    println!("\n{}", "Example 2: Error deeper in parsing".yellow());
    let input2 = "SELECT * FORM users WHERE age > 18";
    let backtrace2 = Backtrace::new();

    println!("Input: {}", input2.red());
    println!("\nParser successfully parses SELECT *, then:");

    // Parser successfully gets past SELECT *
    println!("  Position 0-8: SELECT * parsed ✓");

    // Now tries to parse FROM but finds FORM
    println!("  Position 9: Try FROM... failed (found 'FORM')");
    backtrace2.track_error(9, "FROM", Some("FORM"), input2);

    // Parser might try other things at position 9
    println!("  Position 9: Try comma (for more columns)... failed");
    backtrace2.track_error(9, ",", Some("FORM"), input2);

    // Parser backtracks to position 0 and tries other statement types
    println!("  Backtrack to start, try INSERT INTO... failed");
    backtrace2.track_error(0, "INSERT", Some("SELECT"), input2);

    let error2 = backtrace2.get_error(input2);
    println!("\n{} {}", "Result:".green(), error2.message);
    if let Some(suggestion) = error2.suggestion {
        println!(
            "  {} {} (at position {})",
            "Suggestion:".cyan(),
            suggestion,
            error2.column
        );
    }
    println!(
        "\n{}",
        "Note: Error reported at position 9 (furthest point reached)!".bright_green()
    );

    // Example 3: Multiple errors, furthest wins
    println!(
        "\n{}",
        "Example 3: Multiple errors, furthest position wins".yellow()
    );
    let input3 = "SLECT name FORM users WHEER age > 18";
    let backtrace3 = Backtrace::new();

    println!("Input: {}", input3.red());
    println!("\nParser encounters multiple errors:");

    println!("  Position 0: Expected SELECT, found 'SLECT'");
    backtrace3.track_error(0, "SELECT", Some("SLECT"), input3);

    println!("  Position 11: Expected FROM, found 'FORM'");
    backtrace3.track_error(11, "FROM", Some("FORM"), input3);

    println!(
        "  Position 22: Expected WHERE, found 'WHEER' {}",
        "← Furthest!".bright_green()
    );
    backtrace3.track_error(22, "WHERE", Some("WHEER"), input3);

    let error3 = backtrace3.get_error(input3);
    println!("\n{} {}", "Result:".green(), error3.message);
    if let Some(suggestion) = error3.suggestion {
        println!(
            "  {} {} (ignores earlier errors!)",
            "Suggestion:".cyan(),
            suggestion
        );
    }

    println!(
        "\n{} Furthest error = parser made most progress before failing",
        "Key insight:".bright_cyan().bold()
    );
    println!("This gives users the most relevant error to fix first!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_furthest_error() {
        let backtrace = Backtrace::new();
        let input = "SELECT * FORM users";

        backtrace.track_error(9, "FROM", Some("FORM"), input);
        backtrace.track_error(0, "INSERT", Some("SELECT"), input);

        let error = backtrace.get_error(input);

        assert!(error.message.contains("FROM"));
        assert!(error.message.contains("FORM"));
        assert_eq!(error.suggestion, Some("FROM".to_string()));
    }

    #[test]
    fn test_multiple_errors_furthest_wins() {
        let backtrace = Backtrace::new();
        let input = "SLECT * FORM users WHEER age > 18";

        backtrace.track_error(0, "SELECT", Some("SLECT"), input);
        backtrace.track_error(8, "FROM", Some("FORM"), input);
        backtrace.track_error(19, "WHERE", Some("WHEER"), input);

        let error = backtrace.get_error(input);

        assert!(error.message.contains("WHERE"));
        assert!(error.message.contains("WHEER"));
        assert_eq!(error.suggestion, Some("WHERE".to_string()));
        assert_eq!(error.line, 1);
        assert_eq!(error.column, 20); // position 19 + 1 for column
    }

    #[test]
    fn test_suggestion() {
        assert_eq!(suggest_keyword("SELCT"), Some("SELECT".to_string()));
        assert_eq!(suggest_keyword("FORM"), Some("FROM".to_string()));
        assert_eq!(suggest_keyword("WHEER"), Some("WHERE".to_string()));
        assert_eq!(suggest_keyword("xyz"), None);
    }
}
