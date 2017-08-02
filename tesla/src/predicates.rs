use chrono::Duration;
use expressions::Expression;
use std::string::String;
use std::vec::Vec;

#[derive(Clone, Debug)]
pub enum EventSelection {
    Each,
    First,
    Last,
}

#[derive(Clone, Debug)]
pub enum Aggregator {
    Avg(usize),
    Sum(usize),
    Max(usize),
    Min(usize),
    Count, // TODO add ANY and ALL?
}

#[derive(Clone, Debug)]
pub struct ParameterDeclaration {
    pub name: String,
    pub expression: Expression,
}

#[derive(Clone, Debug)]
pub enum TimingBound {
    Within { window: Duration },
    Between { lower: usize },
}

#[derive(Clone, Debug)]
pub struct Timing {
    pub upper: usize,
    pub bound: TimingBound,
}

#[derive(Clone, Debug)]
pub enum Order {
    Asc,
    Desc,
}

#[derive(Clone, Debug)]
pub struct Ordering {
    pub attribute: usize,
    pub direction: Order,
}

#[derive(Clone, Debug)]
pub enum PredicateType {
    Trigger { parameters: Vec<ParameterDeclaration>, },
    Event {
        selection: EventSelection,
        parameters: Vec<ParameterDeclaration>,
        timing: Timing,
    },
    OrderedStatic {
        // Selection mode always `FIRST`
        // (Last isn't sigificant since you can always specify the opposite ordering)
        parameters: Vec<ParameterDeclaration>,
        ordering: Vec<Ordering>,
    },
    UnorderedStatic {
        // Selection mode always `EACH`
        parameters: Vec<ParameterDeclaration>,
    },
    EventAggregate {
        aggregator: Aggregator,
        parameter: ParameterDeclaration,
        timing: Timing,
    },
    StaticAggregate {
        aggregator: Aggregator,
        parameter: ParameterDeclaration,
    },
    EventNegation { timing: Timing },
    StaticNegation,
}

#[derive(Clone, Debug)]
pub struct ConstrainedTuple {
    pub ty_id: usize,
    pub constraints: Vec<Expression>,
    pub alias: String,
}

#[derive(Clone, Debug)]
pub struct Predicate {
    pub ty: PredicateType,
    pub tuple: ConstrainedTuple,
}

impl PredicateType {
    fn get_used_parameters(&self) -> Vec<(usize, usize)> {
        match *self {
            PredicateType::Trigger { ref parameters } |
            PredicateType::Event { ref parameters, .. } |
            PredicateType::OrderedStatic { ref parameters, .. } |
            PredicateType::UnorderedStatic { ref parameters } => {
                let mut res = parameters.iter()
                    .flat_map(|it| it.expression.get_parameters())
                    .collect::<Vec<_>>();
                res.sort();
                res.dedup();
                res
            }
            PredicateType::EventAggregate { ref parameter, .. } |
            PredicateType::StaticAggregate { ref parameter, .. } => {
                parameter.expression.get_parameters()
            }
            _ => Vec::new(),
        }
    }
}

impl ConstrainedTuple {
    fn get_used_parameters(&self) -> Vec<(usize, usize)> {
        let mut res = self.constraints
            .iter()
            .flat_map(|it| it.get_parameters())
            .collect::<Vec<_>>();
        res.sort();
        res.dedup();
        res
    }
}

impl Predicate {
    pub fn get_used_parameters(&self) -> Vec<(usize, usize)> {
        let mut res = self.ty.get_used_parameters();
        res.append(&mut self.tuple.get_used_parameters());
        res.sort();
        res.dedup();
        res
    }
}
