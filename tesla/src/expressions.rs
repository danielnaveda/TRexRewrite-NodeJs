use ordered_float::NotNaN;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BasicType {
    Int,
    Float,
    Bool,
    Str,
}

#[derive(Clone, Debug)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),
}

// TODO add a RawValue(i32, f32, bool, String)?
// Paying a little cost in memory It would allow unchecked access to values
// But for safety and ergonomy it could be easily converted to/from the Value enum.

#[derive(Clone, Debug)]
pub enum UnaryOperator {
    Minus,
    Not,
}

#[derive(Clone, Debug)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Times,
    Division,
    Equal,
    NotEqual,
    GreaterThan,
    GreaterEqual,
    LowerThan,
    LowerEqual, // TODO add Reminder
}

#[derive(Clone, Debug)]
pub enum Expression {
    Immediate { value: Value },
    /// It always refers to the predicate it appears in
    Reference { attribute: usize },
    /// It refers to the value of the aggregation predicate it appears in
    Aggregate,
    Parameter {
        predicate: usize,
        parameter: usize, // TODO maybe replace with Arc<Expression>
    },
    Cast {
        ty: BasicType,
        expression: Box<Expression>,
    },
    UnaryOperation {
        operator: UnaryOperator,
        expression: Box<Expression>,
    },
    BinaryOperation {
        operator: BinaryOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
}

// TODO think about utility of the following functions

impl Value {
    pub fn unwrap_int(&self) -> i64 {
        if let Value::Int(value) = *self { value } else { panic!("Wrong Value unwrap") }
    }
    pub fn unwrap_float(&self) -> f64 {
        if let Value::Float(value) = *self { value } else { panic!("Wrong Value unwrap") }
    }
    pub fn unwrap_bool(&self) -> bool {
        if let Value::Bool(value) = *self { value } else { panic!("Wrong Value unwrap") }
    }
    pub fn unwrap_string(&self) -> String {
        if let Value::Str(ref value) = *self { value.clone() } else { panic!("Wrong Value unwrap") }
    }
}

impl From<i64> for Value {
    fn from(val: i64) -> Self { Value::Int(val) }
}

impl From<f64> for Value {
    fn from(val: f64) -> Self { Value::Float(val) }
}

impl From<bool> for Value {
    fn from(val: bool) -> Self { Value::Bool(val) }
}

impl From<String> for Value {
    fn from(val: String) -> Self { Value::Str(val) }
}

impl Value {
    pub fn get_type(&self) -> BasicType {
        match *self {
            Value::Int(_) => BasicType::Int,
            Value::Float(_) => BasicType::Float,
            Value::Bool(_) => BasicType::Bool,
            Value::Str(_) => BasicType::Str,
        }
    }
}

impl Hash for Value {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match *self {
            Value::Int(x) => x.hash(state),
            Value::Float(x) => NotNaN::from(x).hash(state),
            Value::Bool(x) => x.hash(state),
            Value::Str(ref x) => x.hash(state),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&Value::Int(x), &Value::Int(y)) => x.eq(&y),
            (&Value::Float(x), &Value::Float(y)) => NotNaN::from(x).eq(&NotNaN::from(y)),
            (&Value::Bool(x), &Value::Bool(y)) => x.eq(&y),
            (&Value::Str(ref x), &Value::Str(ref y)) => x.eq(y),
            _ => false,
        }
    }
}
impl Eq for Value {}

impl Expression {
    pub fn is_local(&self) -> bool {
        // TODO maybe take into account local parameters that don't alter expression locality
        match *self {
            Expression::Parameter { .. } => false,
            Expression::Cast { ref expression, .. } |
            Expression::UnaryOperation { ref expression, .. } => expression.is_local(),
            Expression::BinaryOperation { ref left, ref right, .. } => {
                left.is_local() && right.is_local()
            }
            _ => true,
        }
    }

    pub fn get_parameters(&self) -> Vec<(usize, usize)> {
        match *self {
            Expression::Parameter { predicate, parameter } => vec![(predicate, parameter)],
            Expression::Cast { ref expression, .. } |
            Expression::UnaryOperation { ref expression, .. } => expression.get_parameters(),
            Expression::BinaryOperation { ref left, ref right, .. } => {
                let mut res = left.get_parameters();
                res.append(&mut right.get_parameters());
                res.sort();
                res.dedup();
                res
            }
            _ => Vec::new(),
        }
    }

    pub fn get_last_predicate(&self) -> Option<usize> {
        self.get_parameters().last().map(|&(pred, _)| pred)
    }
}
