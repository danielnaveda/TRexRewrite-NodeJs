pub mod unary {
    use tesla::expressions::{BasicType, UnaryOperator, Value};

    fn get_type_minus(ty: &BasicType) -> Result<BasicType, String> {
        match *ty {
            BasicType::Int | BasicType::Float => Ok(ty.clone()),
            _ => Err("Wrong operand type in unary minus".to_owned()),
        }
    }

    fn get_type_not(ty: &BasicType) -> Result<BasicType, String> {
        if let BasicType::Bool = *ty {
            Ok(ty.clone())
        } else {
            Err("Wrong operand type in unary not".to_owned())
        }
    }

    pub fn get_type(operator: &UnaryOperator, ty: &BasicType) -> Result<BasicType, String> {
        match *operator {
            UnaryOperator::Minus => get_type_minus(ty),
            UnaryOperator::Not => get_type_not(ty),
        }
    }

    fn evaluate_minus(value: &Value) -> Value {
        match *value {
            Value::Int(x) => Value::Int(-x),
            Value::Float(x) => Value::Float(-x),
            _ => panic!("Wrong use of unary minus"),
        }
    }

    fn evaluate_not(value: &Value) -> Value {
        match *value {
            Value::Bool(x) => Value::Bool(!x),
            _ => panic!("Wrong use of not operator"),
        }
    }

    pub fn evaluate(operator: &UnaryOperator, value: &Value) -> Value {
        match *operator {
            UnaryOperator::Minus => evaluate_minus(value),
            UnaryOperator::Not => evaluate_not(value),
        }
    }
}

pub mod binary {
    use std::f64::EPSILON;
    use tesla::expressions::{BasicType, BinaryOperator, Value};

    fn get_type_arithmetic(left: &BasicType, right: &BasicType) -> Result<BasicType, String> {
        match (left, right) {
            (&BasicType::Int, &BasicType::Int) => Ok(BasicType::Int),
            (&BasicType::Float, &BasicType::Float) => Ok(BasicType::Float),
            _ => Err("Wrong operands types in arithmetic operation".to_owned()),
        }
    }

    fn get_type_equality(left: &BasicType, right: &BasicType) -> Result<BasicType, String> {
        if left == right {
            Ok(BasicType::Bool)
        } else {
            Err("Wrong operands types in equality operation".to_owned())
        }
    }

    fn get_type_comparison(left: &BasicType, right: &BasicType) -> Result<BasicType, String> {
        match (left, right) {
            (&BasicType::Int, &BasicType::Int) |
            (&BasicType::Float, &BasicType::Float) |
            (&BasicType::Str, &BasicType::Str) => Ok(BasicType::Bool),
            _ => Err("Wrong operands types in comparison operation".to_owned()),
        }
    }

    pub fn get_type(operator: &BinaryOperator,
                    left: &BasicType,
                    right: &BasicType)
                    -> Result<BasicType, String> {
        match *operator {
            BinaryOperator::Plus | BinaryOperator::Minus | BinaryOperator::Times |
            BinaryOperator::Division => get_type_arithmetic(left, right),
            BinaryOperator::Equal | BinaryOperator::NotEqual => get_type_equality(left, right),
            BinaryOperator::GreaterThan |
            BinaryOperator::GreaterEqual |
            BinaryOperator::LowerThan |
            BinaryOperator::LowerEqual => get_type_comparison(left, right),
        }
    }

    fn evaluate_plus(left: &Value, right: &Value) -> Value {
        match (left, right) {
            (&Value::Int(lhs), &Value::Int(rhs)) => Value::Int(lhs + rhs),
            (&Value::Float(lhs), &Value::Float(rhs)) => Value::Float(lhs + rhs),
            (&Value::Str(ref lhs), &Value::Str(ref rhs)) => Value::Str(format!("{}{}", lhs, rhs)),
            _ => panic!("Wrong use of plus operator"),
        }
    }

    fn evaluate_minus(left: &Value, right: &Value) -> Value {
        match (left, right) {
            (&Value::Int(lhs), &Value::Int(rhs)) => Value::Int(lhs - rhs),
            (&Value::Float(lhs), &Value::Float(rhs)) => Value::Float(lhs - rhs),
            _ => panic!("Wrong use of minus operator"),
        }
    }

    fn evaluate_times(left: &Value, right: &Value) -> Value {
        match (left, right) {
            (&Value::Int(lhs), &Value::Int(rhs)) => Value::Int(lhs * rhs),
            (&Value::Float(lhs), &Value::Float(rhs)) => Value::Float(lhs * rhs),
            _ => panic!("Wrong use of times operator"),
        }
    }

    fn evaluate_division(left: &Value, right: &Value) -> Value {
        match (left, right) {
            (&Value::Int(lhs), &Value::Int(rhs)) => Value::Int(lhs / rhs),
            (&Value::Float(lhs), &Value::Float(rhs)) => Value::Float(lhs / rhs),
            _ => panic!("Wrong use of division operator"),
        }
    }

    fn evaluate_equal(left: &Value, right: &Value) -> Value {
        match (left, right) {
            (&Value::Int(lhs), &Value::Int(rhs)) => Value::Bool(lhs == rhs),
            (&Value::Float(lhs), &Value::Float(rhs)) => Value::Bool((lhs - rhs).abs() < EPSILON),
            (&Value::Bool(lhs), &Value::Bool(rhs)) => Value::Bool(lhs == rhs),
            (&Value::Str(ref lhs), &Value::Str(ref rhs)) => Value::Bool(lhs == rhs),
            _ => panic!("Wrong use of equal operator"),
        }
    }

    fn evaluate_not_equal(left: &Value, right: &Value) -> Value {
        match (left, right) {
            (&Value::Int(lhs), &Value::Int(rhs)) => Value::Bool(lhs != rhs),
            (&Value::Float(lhs), &Value::Float(rhs)) => Value::Bool((lhs - rhs).abs() >= EPSILON),
            (&Value::Bool(lhs), &Value::Bool(rhs)) => Value::Bool(lhs != rhs),
            (&Value::Str(ref lhs), &Value::Str(ref rhs)) => Value::Bool(lhs != rhs),
            _ => panic!("Wrong use of not_equal operator"),
        }
    }

    fn evaluate_greater_than(left: &Value, right: &Value) -> Value {
        match (left, right) {
            (&Value::Int(lhs), &Value::Int(rhs)) => Value::Bool(lhs > rhs),
            (&Value::Float(lhs), &Value::Float(rhs)) => Value::Bool(lhs - rhs >= EPSILON),
            (&Value::Str(ref lhs), &Value::Str(ref rhs)) => Value::Bool(lhs > rhs),
            _ => panic!("Wrong use of greater_than operator"),
        }
    }

    fn evaluate_greater_equal(left: &Value, right: &Value) -> Value {
        match (left, right) {
            (&Value::Int(lhs), &Value::Int(rhs)) => Value::Bool(lhs >= rhs),
            (&Value::Float(lhs), &Value::Float(rhs)) => Value::Bool(lhs - rhs > -EPSILON),
            (&Value::Str(ref lhs), &Value::Str(ref rhs)) => Value::Bool(lhs >= rhs),
            _ => panic!("Wrong use of greater_equal operator"),
        }
    }

    fn evaluate_lower_than(left: &Value, right: &Value) -> Value {
        match (left, right) {
            (&Value::Int(lhs), &Value::Int(rhs)) => Value::Bool(lhs < rhs),
            (&Value::Float(lhs), &Value::Float(rhs)) => Value::Bool(rhs - lhs >= EPSILON),
            (&Value::Str(ref lhs), &Value::Str(ref rhs)) => Value::Bool(lhs < rhs),
            _ => panic!("Wrong use of lower_than operator"),
        }
    }

    fn evaluate_lower_equal(left: &Value, right: &Value) -> Value {
        match (left, right) {
            (&Value::Int(lhs), &Value::Int(rhs)) => Value::Bool(lhs <= rhs),
            (&Value::Float(lhs), &Value::Float(rhs)) => Value::Bool(rhs - lhs > -EPSILON),
            (&Value::Str(ref lhs), &Value::Str(ref rhs)) => Value::Bool(lhs <= rhs),
            _ => panic!("Wrong use of lower_equal operator"),
        }
    }

    pub fn evaluate(operator: &BinaryOperator, left: &Value, right: &Value) -> Value {
        match *operator {
            BinaryOperator::Plus => evaluate_plus(left, right),
            BinaryOperator::Minus => evaluate_minus(left, right),
            BinaryOperator::Times => evaluate_times(left, right),
            BinaryOperator::Division => evaluate_division(left, right),
            BinaryOperator::Equal => evaluate_equal(left, right),
            BinaryOperator::NotEqual => evaluate_not_equal(left, right),
            BinaryOperator::GreaterThan => evaluate_greater_than(left, right),
            BinaryOperator::GreaterEqual => evaluate_greater_equal(left, right),
            BinaryOperator::LowerThan => evaluate_lower_than(left, right),
            BinaryOperator::LowerEqual => evaluate_lower_equal(left, right),
        }
    }
}
