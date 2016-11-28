use FnvHashMap;
use chrono::{DateTime, UTC};
use expressions::evaluation::*;
use linear_map::LinearMap;
use std::sync::Arc;
use tesla::{Event, Rule, Tuple};
use tesla::expressions::Value;
use tesla::predicates::*;

#[derive(Clone, Debug)]
pub struct PartialResult {
    parameters: LinearMap<(usize, usize), Value>,
    events: LinearMap<usize, Arc<Event>>,
}

impl PartialResult {
    pub fn new() -> Self {
        PartialResult {
            parameters: LinearMap::new(),
            events: LinearMap::new(),
        }
    }

    #[inline(always)]
    pub fn insert_event(mut self, idx: usize, event: Arc<Event>) -> Self {
        self.events.insert(idx, event);
        self
    }

    #[inline(always)]
    pub fn insert_parameter(mut self, idx: (usize, usize), parameter: Value) -> Self {
        self.parameters.insert(idx, parameter);
        self
    }

    pub fn get_parameter(&self, idx: (usize, usize)) -> &Value { &self.parameters[&idx] }

    #[inline(always)]
    pub fn get_time(&self, idx: usize) -> DateTime<UTC> { self.events[&idx].time }
}

pub trait EventProcessor: Send {
    #[allow(unused_variables)]
    fn process(&mut self, event: &Arc<Event>) {}
    #[allow(unused_variables)]
    fn consume(&mut self, event: &Arc<Event>) {}
    #[allow(unused_variables)]
    fn remove_old(&mut self, times: &FnvHashMap<usize, DateTime<UTC>>) -> Option<DateTime<UTC>> {
        None
    }
    fn evaluate(&self, result: &PartialResult) -> Vec<PartialResult>;
}

#[derive(Clone, Debug)]
pub struct Trigger {
    predicate: Predicate,
}

impl Trigger {
    pub fn new(predicate: &Predicate) -> Self { Trigger { predicate: predicate.clone() } }

    fn is_satisfied(&self, context: &CompleteContext) -> bool {
        let check_expr = |expr: &Arc<_>| context.evaluate_expression(expr).unwrap_bool();
        self.predicate.tuple.constraints.iter().all(check_expr)
    }

    fn evaluate(&self, event: &Arc<Event>) -> Option<PartialResult> {
        if event.tuple.ty_id == self.predicate.tuple.ty_id {
            let res = if let PredicateType::Trigger { ref parameters } = self.predicate.ty {
                parameters.iter().enumerate().fold(PartialResult::new(), |res, (i, param)| {
                    let val = CompleteContext::new(&res, &event.tuple)
                        .evaluate_expression(&param.expression);
                    res.insert_parameter((0, i), val)
                })
            } else {
                panic!("Unexpected predicate type")
            };
            if self.is_satisfied(&CompleteContext::new(&res, &event.tuple)) {
                Some(res.insert_event(0, event.clone()))
            } else {
                None
            }
        } else {
            None
        }
    }
}

pub struct RuleStacks {
    trigger: Trigger,
    processors: LinearMap<usize, Box<EventProcessor>>,
    rule: Rule,
}

impl RuleStacks {
    pub fn new(trigger: Trigger,
               processors: LinearMap<usize, Box<EventProcessor>>,
               rule: Rule)
               -> Self {
        RuleStacks {
            trigger: trigger,
            processors: processors,
            rule: rule,
        }
    }

    fn remove_old_events(&mut self, trigger_time: &DateTime<UTC>) {
        let mut times = FnvHashMap::default();
        times.insert(0, *trigger_time);
        for (&i, processor) in &mut self.processors {
            let time = processor.remove_old(&times).unwrap_or(*trigger_time);
            times.insert(i, time);
        }
    }

    fn get_partial_results(&self, initial: PartialResult) -> Vec<PartialResult> {
        self.processors
            .iter()
            .fold(vec![initial], |previous, (_, evaluator)| {
                previous.iter().flat_map(|res| evaluator.evaluate(res)).collect()
                // TODO maybe interrupt fold if prev is empty (combo scan + take_while + last)
            })
    }

    fn generate_events<'a, T>(&self, event: &Arc<Event>, results: T) -> Vec<Arc<Event>>
        where T: IntoIterator<Item = &'a PartialResult>
    {
        results.into_iter()
            .map(|res| {
                let context = CompleteContext::new(res, ());
                let template = &self.rule.event_template;
                Arc::new(Event {
                    tuple: Tuple {
                        ty_id: template.ty_id,
                        data: template.attributes
                            .iter()
                            .map(|expr| context.evaluate_expression(expr))
                            .collect(),
                    },
                    time: event.time,
                })
            })
            .collect()
    }

    pub fn process(&mut self, event: &Arc<Event>) -> Vec<Arc<Event>> {
        for (_, processor) in &mut self.processors {
            processor.process(event);
        }

        if let Some(initial) = self.trigger.evaluate(event) {
            self.remove_old_events(&event.time);
            let partial_results = self.get_partial_results(initial);
            // TODO move filter as early as possible in the partial_results generation
            let filtered = partial_results.iter().filter(|res| {
                let context = CompleteContext::new(res, ());
                self.rule
                    .filters
                    .iter()
                    .all(|expr| context.evaluate_expression(expr).unwrap_bool())
            });
            // TODO consuming clause
            self.generate_events(event, filtered)
        } else {
            Vec::new()
        }
    }
}
