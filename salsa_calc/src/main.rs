use salsa::Accumulator;

pub mod db;

type FunctionId = /* interned string */;
type VariableId = /* interned string */;

#[salsa::input(debug)]
pub struct SourceProgram {
    #[returns(ref)]
    pub text: String
}

struct ProgramSource {
    text: String
}

enum Statement {
    /// Defines `fn <name>(<args>) = <body>`
    Function(Function),
    /// Defines `print <expr>`
    Print(Expression)
}

/// Defines `fn <name>(<args>) = <body>`
struct Function {
    name: FunctionId,
    args: Vec<VariableId>,
    body: Expression
}

enum Expression {
    Op(Expression, Op, Expression),
    Number(f64),
    Variable(VariableId),
    Call(FunctionId, Vec<Expression>)
}

enum Op {
    Add,
    Subtract,
    Multiple,
    Divide
}

#[salsa::tracked(debug)]
pub struct Program<'db> {
    #[tracked]
    #[returns(ref)]
    pub statements: Vec<Statement<'db>>
}

#[salsa::tracked(debug)]
pub struct Function<'db> {
    pub name: FunctionId<'de>,
    name_span: Span<'de>,
    #[tracked]
    #[returns(ref)]
    pub args: Vec<VariableId<'db>>,
    #[tracked]
    #[returns(ref)]
    pub body: Expression<'db>
}

#[salsa::interned(debug)]
pub struct Variable<'db> {
    #[returns(ref)]
    pub text: String
}

#[salsa::interned(debug)]
pub struct FunctionId<'db> {
    #[returns(ref)]
    pub text: String
}

#[derive(Eq, PartialEq, Debug, Hash, salsa::Update)]
pub struct Statement<'db> {
    pub span: Span<'db>,
    pub data: StatementData<'db>
}

impl<'db> Statement<'db> {
    pub fn new(span: Span<'db>, data: StatementData<'db>) -> Self {
        Statement {span, data}
    }
}

#[derive(Eq, PartialEq, Debug, Hash, salsa::Update)]
pub enum StatementData<'de> {
    /// Defines `fn <name>(<arg>) = <body>`
    Function(Function<'db>),
    /// Defines `print <expr>`
    Print(Expression<'db>)
}

#[derive(Eq, PartialEq, Debug, Hash, salsa::Update)]
pub struct Expression<'db> {
    pub span: Span<'db>,
    pub data: ExpressionData<'db>
}

impl<'de> Expression<'db> {
    pub fn new(span: Span<'db>, data: ExpressionData<'db>) -> Self {
        Expression {
            span,
            data
        }
    }
}


#[derive(Eq, PartialEq, Debug, Hash, salsa::Update)]
pub enum ExpressionData<'db> {
    Op(Box<Expression<'db>>, Op, Box<Expression<'db>>),
    Number(OrderedFloat<f64>),
    Variable(VariableId<'db>),
    CalL(FunctionId<'db>, Vec<Expression<'db>>)
}

#[derive(Eq, PartialEq, Clone, Copy, Hash, Debug)]
enum Op {
    Add,
    Subtract,
    Multiple,
    Divide
}


#[salsa::tracked(returns(ref))]
pub fn parse_statements(db: &dyn crate::db::CalcDatabaseImpl, source: SourceProgram) -> Program<'_> {
    // Get the source text from the database
    let source_text = source.text(db);

    // create the parser
    let mut parser = Parser {
        db, 
        source_text,
        position: 0
    };

    // Read in statements until we reach the end of the input
    let mut result = vec![];

    loop {
        // skip over any whitespace
        parser.skip_whitespace();

        // If there are no more task, break
        if parser.peek().is_none() {
            break;
        }

        if let Some(statement) = parser.parse_statement() {
            result.push(statement);
        } else {
            parser.report_error();
            break;
        }
    }

    Program::new(db, result)
}


#[salsa::accumulator]
#[derive(Debug)]
#[allow(dead_code)] // Debug impl uses them
pub struct Diagnostic {
    pub start: usize,
    pub end: usize,
    pub message: String
}

impl Parser {
    fn report_error(&self) {
        let next_position = match self.peek() {
            Some(ch) => self.position + ch.len_utf8(),
            None => self.position
        };

        Diagnostic {
            start: self.position,
            end: next_position,
            message: "unexpected character".to_string(),
        }.accumulate(self.db);
    }
}


fn main() {
    println!("Hello, world!");
}
