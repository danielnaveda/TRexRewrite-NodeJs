use {FnvHashMap, NodeProvider};
use aggregators::compute_aggregate;
use chrono::{DateTime, UTC};
use expressions::evaluation::*;
use linear_map::LinearMap;
use rule_processor::{EventProcessor, PartialResult};
use std::cmp::Ordering as CmpOrd;
use std::sync::Arc;
use tesla::{Event, TupleDeclaration};
use tesla::expressions::*;
use tesla::predicates::*;

fn ptr_eq<T>(a: *const T, b: *const T) -> bool { a == b }

#[derive(Clone, Debug)]
pub struct Stack {
    idx: usize,
    tuple: TupleDeclaration,
    predicate: Predicate,
    local_exprs: Vec<Expression>,
    global_exprs: Vec<Expression>,
    timing: Timing,
    events: Vec<Arc<Event>>,
}

impl Stack {
    pub fn new(idx: usize, tuple: &TupleDeclaration, predicate: &Predicate) -> Option<Stack> {
        match predicate.ty {
            PredicateType::Event { ref timing, .. } |
            PredicateType::EventAggregate { ref timing, .. } |
            PredicateType::EventNegation { ref timing } => {
                let (local_exprs, global_exprs) = predicate.tuple
                    .constraints
                    .iter()
                    .cloned()
                    .partition(|expr| expr.is_local());

                Some(Stack {
                    idx: idx,
                    tuple: tuple.clone(),
                    predicate: predicate.clone(),
                    local_exprs: local_exprs,
                    global_exprs: global_exprs,
                    timing: timing.clone(),
                    events: Vec::new(),
                })
            }
            _ => None,
        }
    }

    fn is_locally_satisfied(&self, event: &Arc<Event>) -> bool {
        event.tuple.ty_id == self.predicate.tuple.ty_id &&
        {
            let context = SimpleContext::new(&event.tuple);
            let check_expr = |expr| context.evaluate_expression(expr).unwrap_bool();
            self.local_exprs.iter().all(check_expr)
        }
    }

    fn is_globally_satisfied(&self, context: &CompleteContext) -> bool {
        let check_expr = |expr| context.evaluate_expression(expr).unwrap_bool();
        self.global_exprs.iter().all(check_expr)
    }
}

impl EventProcessor for Stack {
    fn process(&mut self, event: &Arc<Event>) {
        if self.is_locally_satisfied(event) {
            // TODO reason on precondition: all the events arrive in chronological order
            self.events.push(event.clone());
        }
    }

    fn consume(&mut self, event: &Arc<Event>) {
        let index = {
            let start = self.events
                .binary_search_by(|evt| {
                    if evt.time < event.time { CmpOrd::Less } else { CmpOrd::Greater }
                })
                .unwrap_err();
            // TODO handle the absence of the event from the queue
            self.events[start..].iter().position(|evt| ptr_eq(evt, event)).unwrap() + start
        };
        self.events.remove(index);
    }

    fn remove_old(&mut self, times: &FnvHashMap<usize, DateTime<UTC>>) -> Option<DateTime<UTC>> {
        // TODO reason on interval (open vs close)
        let time = match self.timing.bound {
            TimingBound::Within { window } => times[&self.timing.upper] - window,
            TimingBound::Between { lower } => times[&lower],
        };

        let index = self.events
            .binary_search_by(|evt| {
                if evt.time < time { CmpOrd::Less } else { CmpOrd::Greater }
            })
            .unwrap_err();
        self.events.drain(..index);

        self.events.first().map(|evt| evt.time)
    }

    fn evaluate(&self, result: &PartialResult) -> Vec<PartialResult> {
        let upper_time = result.get_time(self.timing.upper);
        let lower_time = match self.timing.bound {
            TimingBound::Within { window } => upper_time - window,
            TimingBound::Between { lower } => result.get_time(lower),
        };

        let upper = self.events
            .binary_search_by(|evt| {
                if evt.time < upper_time { CmpOrd::Less } else { CmpOrd::Greater }
            })
            .unwrap_err();
        let lower = self.events
            .binary_search_by(|evt| {
                if evt.time < lower_time { CmpOrd::Less } else { CmpOrd::Greater }
            })
            .unwrap_err();

        let mut iterator = self.events[lower..upper].iter();

        match self.predicate.ty {
            PredicateType::Event { ref selection, ref parameters, .. } => {
                let filter_map = |evt: &Arc<Event>| {
                    let res =
                        parameters.iter().enumerate().fold(result.clone(), |res, (i, param)| {
                            let val = CompleteContext::new(&res, &evt.tuple)
                                .evaluate_expression(&param.expression);
                            res.insert_parameter((self.idx, i), val)
                        });
                    if self.is_globally_satisfied(&CompleteContext::new(&res, &evt.tuple)) {
                        Some(res.insert_event(self.idx, evt.clone()))
                    } else {
                        None
                    }
                };
                match *selection {
                    EventSelection::Each => iterator.filter_map(filter_map).collect(),
                    EventSelection::First => iterator.filter_map(filter_map).take(1).collect(),
                    EventSelection::Last => iterator.rev().filter_map(filter_map).take(1).collect(),
                }
            }
            PredicateType::EventAggregate { ref aggregator, ref parameter, .. } => {
                let check = |evt: &&Arc<Event>| {
                    self.is_globally_satisfied(&CompleteContext::new(result, &evt.tuple))
                };
                let map = |aggr: Value| {
                    let context = CompleteContext::new(result, &aggr);
                    let val = context.evaluate_expression(&parameter.expression);
                    result.clone().insert_parameter((self.idx, 0), val)
                };
                compute_aggregate(aggregator, iterator.filter(check), &self.tuple.attributes)
                    .map(map)
                    .into_iter()
                    .collect()
            }
            PredicateType::EventNegation { .. } => {
                let check = |evt: &Arc<Event>| {
                    self.is_globally_satisfied(&CompleteContext::new(result, &evt.tuple))
                };
                if !iterator.any(check) { vec![result.clone()] } else { Vec::new() }
            }
            _ => panic!("Wrong event stack evaluation"),
        }
    }
}

pub struct StackProvider;

impl NodeProvider for StackProvider {
    fn provide(&self,
               idx: usize,
               tuple: &TupleDeclaration,
               predicate: &Predicate,
               _: &LinearMap<(usize, usize), BasicType>)
               -> Option<Box<EventProcessor>> {
        Stack::new(idx, tuple, predicate).map(|it| Box::new(it) as Box<EventProcessor>)
    }
}
