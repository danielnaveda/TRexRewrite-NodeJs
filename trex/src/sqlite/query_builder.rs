use tesla::*;
use tesla::expressions::*;
use tesla::predicates::*;

pub struct SqlContext<'a> {
    idx: usize,
    tuple: &'a TupleDeclaration,
    parameters: Vec<String>,
    aggregate: Option<String>,
}

impl<'a> SqlContext<'a> {
    pub fn new(idx: usize, tuple: &'a TupleDeclaration) -> Self {
        SqlContext {
            idx: idx,
            tuple: tuple,
            parameters: Vec::new(),
            aggregate: None,
        }
    }

    fn set_aggregate(&mut self, aggr: &Aggregator) {
        let sql = match *aggr {
            Aggregator::Avg(attribute) => {
                format!("AVG({}.{})",
                        self.tuple.name,
                        self.tuple.attributes[attribute].name)
            }
            Aggregator::Sum(attribute) => {
                format!("SUM({}.{})",
                        self.tuple.name,
                        self.tuple.attributes[attribute].name)
            }
            Aggregator::Max(attribute) => {
                format!("MAX({}.{})",
                        self.tuple.name,
                        self.tuple.attributes[attribute].name)
            }
            Aggregator::Min(attribute) => {
                format!("MIN({}.{})",
                        self.tuple.name,
                        self.tuple.attributes[attribute].name)
            }
            Aggregator::Count => "COUNT(*)".to_owned(),
        };
        self.aggregate = Some(sql);
    }

    fn insert_parameter(&mut self, param: &ParameterDeclaration) -> String {
        let sql = self.encode_expression(&param.expression);
        self.parameters.push(sql.clone());
        sql
    }

    fn encode_value(&self, value: &Value) -> String {
        match *value {
            Value::Int(value) => format!("{}", value),
            Value::Float(value) => format!("{}", value),
            Value::Bool(value) => format!("{}", value),
            // TODO check excaping for SQL injection
            Value::Str(ref value) => format!("{:?}", value),
        }
    }

    fn encode_unary(&self, op: &UnaryOperator) -> String {
        match *op {
                UnaryOperator::Minus => "-",
                UnaryOperator::Not => "!",
            }
            .to_owned()
    }

    fn encode_binary(&self, op: &BinaryOperator) -> String {
        match *op {
                BinaryOperator::Plus => "+",
                BinaryOperator::Minus => "-",
                BinaryOperator::Times => "*",
                BinaryOperator::Division => "/",
                BinaryOperator::Equal => "=",
                BinaryOperator::NotEqual => "!=",
                BinaryOperator::GreaterThan => ">",
                BinaryOperator::GreaterEqual => ">=",
                BinaryOperator::LowerThan => "<",
                BinaryOperator::LowerEqual => "<=",
            }
            .to_owned()
    }

    fn encode_attribute(&self, attribute: usize) -> String {
        format!("{}.{}",
                self.tuple.name,
                self.tuple.attributes[attribute].name)
    }

    fn get_parameter(&self, predicate: usize, parameter: usize) -> String {
        if predicate == self.idx {
            self.parameters[parameter].clone()
        } else {
            format!(":param{}x{}", predicate, parameter)
        }
    }

    fn encode_expression(&self, expr: &Expression) -> String {
        match *expr {
            Expression::Immediate { ref value } => self.encode_value(value),
            Expression::Reference { attribute } => self.encode_attribute(attribute),
            Expression::Parameter { predicate, parameter } => {
                self.get_parameter(predicate, parameter)
            }
            Expression::Aggregate => self.aggregate.clone().unwrap(),
            Expression::Cast { ref expression, .. } => self.encode_expression(expression),
            Expression::UnaryOperation { ref operator, ref expression } => {
                format!("({}{})",
                        self.encode_unary(operator),
                        self.encode_expression(expression))
            }
            Expression::BinaryOperation { ref operator, ref left, ref right } => {
                format!("({} {} {})",
                        self.encode_expression(left),
                        self.encode_binary(operator),
                        self.encode_expression(right))
            }
        }
    }

    fn encode_order(&self, ord: &Order) -> String {
        match *ord {
                Order::Asc => "ASC",
                Order::Desc => "DESC",
            }
            .to_owned()
    }

    fn encode_ordering(&self, ord: &Ordering) -> String {
        format!("{}.{} {}",
                self.tuple.name,
                self.tuple.attributes[ord.attribute].name,
                self.encode_order(&ord.direction))
    }

    pub fn encode_predicate(&mut self, pred: &Predicate) -> String {
        let selection;
        let filters = pred.tuple
            .constraints
            .iter()
            .map(|expr| self.encode_expression(expr))
            .collect::<Vec<_>>()
            .join(" AND ");
        let mut rest = String::new();

        match pred.ty {
            PredicateType::OrderedStatic { ref parameters, ref ordering } => {
                selection = parameters.iter()
                    .map(|par| {
                        let sql = self.insert_parameter(par);
                        format!("{} AS {}", sql, par.name)
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                let order_by = ordering.iter()
                    .map(|ord| self.encode_ordering(ord))
                    .collect::<Vec<_>>()
                    .join(", ");
                rest = format!("ORDER BY {} LIMIT 1", order_by);
            }
            PredicateType::UnorderedStatic { ref parameters } => {
                selection = parameters.iter()
                    .map(|par| {
                        let sql = self.insert_parameter(par);
                        format!("{} AS {}", sql, par.name)
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
            }
            PredicateType::StaticAggregate { ref aggregator, ref parameter } => {
                self.set_aggregate(aggregator);
                let sql = self.insert_parameter(parameter);
                selection = format!("{} AS {}", sql, parameter.name);
            }
            PredicateType::StaticNegation => {
                selection = "1".to_owned();
                rest = "LIMIT 1".to_owned();
            }
            _ => panic!("Error composing the SQL statement"),
        }

        // FIXME guard against an empty selection!
        format!("SELECT {} FROM {} WHERE {} {}",
                if !selection.is_empty() { selection } else { "COUNT(1)".to_owned() },
                self.tuple.name,
                filters,
                rest)
    }
}
