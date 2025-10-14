

pub enum ScalarExpr {
    ColumnRef(Vec<Identifier>, Type),
    Literal(Value, Type),
    Function(Signature, Vec<Self>),
    Subquery(Quantifier, Box<Operator>)
}

pub enum Operator {
    Get { table: String, schema: Vec<ColumnDefinition> },
    Select { child: Box<Self>, predicate: ScalarExpr },
    Project { child: Box<Self>, projects: Vec<(ScalarExpr, String)> },
    Join {
        kind: JoinKind,
        condition: ScalarExpr,
        left: Box<Self>,
        right: Box<Self>,
    },
    UnionAll {
        left: Box<Self>,
        right: Box<Self>,
    },
    Aggregate {
        group_by: Vec<ScalarExpr>,
        aggr_exprs: Vec<ScalarExpr>,
        child: Box<Self>,
    }
}

impl Operator {
    fn calculate_attributes(&self, children: &[Vec<ColumnDefinition>]) -> Vec<ColumnDefinition> {
        match self {
            Operator::Get {  schema, .. } => {
                let attributes = schema.clone();
                attributes
            }
            Operator::Select { .. } => children[0],clone(),
            Operator::Join { .. } => {
                let mut attributes = children[0].clone();
                attributes.extend(children[1].clone());
                attributes
            }
            Operator::UnionAll { .. } => children[0].clone(),
            Operator::Project { projects, .. } => {
                let attributes: Vec<ColumnDefinition> = projects.iter().map(|(expr, alias)| ColumnDefinition {
                    name: alias.clone(),
                    column_type: expr.get_type(),
                    not_null: false, // TODO: derive not_null from expr
                }).collect();
                attributes
            },
            Operator::Aggregate { group_by,.. } => {
                let mut attributes: Vec<ColumnDefinition> = group_by.iter().map(|expr| ColumnDefinition {
                    name: expr.name(), // TODO: derive name from expr
                    column_type: expr.get_type(),
                    not_null: false, // TODO: derive not_null from expr
                }).collect();
                attributes.extend(aggr_exprs.iter().map(|expr| ColumnDefinition {
                    name: expr.name(),
                    column_type: expr.column_type(),
                    not_null: expr.nullable(),
                }));

                attributes
            },
        }
    }
}

pub struct QualifiedName(pub Vec<String>);

impl QualifiedName {
    /// If the current name can be used to refer another name
    pub fn can_refer(&self, other: &Self) -> bool {
        self.0.len() <= other.0.len() 
          && self.0.iter().zip(other.0.iter()).all(|(a, b)| a == b)
    }
}
pub struct TableSchema {
    pub name: String,
    pub columns: Vec<ColumnDefinition>
}

pub struct ColumnDefinition {
    pub name: QualifiedName,
    pub column_type: Type,
    pub not_null: bool,
}


fn derive_attributes(op: &Operator) -> Vec<ColumnDefinition> {
    let children_attributes: Vec<Vec<ColumnDefinition>> = op.children().map(derive_attributes).collect();

    op.calculate_attributes(&children_attributes)
}


fn derive_property(op: &Operator) -> Property {
    let children_property: Vec<Property> = op.children().map(derive_property).collect();
    op.calculate_property(&children_property)
}

fn plan() -> Operator {
    Operator::Project {
        child: Box::new(Operator::Select {
            child: Box::new(Operator::Get {
                table: "t".to_string(),
            }),
            predicate: "a = 1".to_string(),
        }),
        projects: "a".to_string(),
    }
}

fn main() {
    println!("Hello, world!");
}
