extern crate chrono;
extern crate num_cpus;
extern crate tesla;
extern crate trex;

use chrono::{Duration, UTC};
use std::sync::Arc;
use tesla::{AttributeDeclaration, Engine, Event, EventTemplate, Rule, Tuple, TupleDeclaration,
            TupleType};
use tesla::expressions::{BasicType, BinaryOperator, Expression, Value};
use tesla::predicates::{ConstrainedTuple, EventSelection, ParameterDeclaration, Predicate,
                        PredicateType, Timing, TimingBound};
use trex::TRex;
use trex::listeners::DebugListener;
use trex::stack::StackProvider;

fn main() {
    // Providers are a factory abstraction, that instantiate
    // an event processor given a rule predicate.
    //
    // In this example is just the one that take care of
    // event predicates, because I used only those.
    // Otherwise there should be one for each used data source,
    // for example `SqliteProvider` is the only other
    // implementation at the moment.
    let provider = Box::new(StackProvider);

    // TRex engine instantiation with the number of threads
    // and a vector of providers.
    let mut engine = TRex::new(num_cpus::get(), vec![provider]);

    // First of all we declare all the events types
    //
    // `declare smoke(area: string) with id 0`
    engine.declare(TupleDeclaration {
        ty: TupleType::Event,
        id: 0,
        name: "smoke".to_owned(),
        attributes: vec![
                AttributeDeclaration {
                    name: "area".to_owned(),
                    ty: BasicType::Str,
                },
            ],
    });

    // `declare temperature(area: string, value: integer) with id 1`
    engine.declare(TupleDeclaration {
        ty: TupleType::Event,
        id: 1,
        name: "temperature".to_owned(),
        attributes: vec![
                AttributeDeclaration {
                    name: "area".to_owned(),
                    ty: BasicType::Str,
                },
                AttributeDeclaration {
                    name: "value".to_owned(),
                    ty: BasicType::Int,
                },
            ],
    });

    // `declare fire(area: string, temp: integer) with id 2`
    engine.declare(TupleDeclaration {
        ty: TupleType::Event,
        id: 2,
        name: "fire".to_owned(),
        attributes: vec![
                AttributeDeclaration {
                    name: "area".to_owned(),
                    ty: BasicType::Str,
                },
                AttributeDeclaration {
                    name: "temp".to_owned(),
                    ty: BasicType::Int,
                },
            ],
    });

    // THen we define a rule over those declarations
    //
    // `from smoke[$x = area]() as smk
    //  and last temperature[$y = value](area == $x, value > 45) as temp within 5min from smk
    //  emit fire(area = $x, temp = $y)`
    engine.define(Rule {
        predicates: vec![
            Predicate {
                ty: PredicateType::Trigger {
                    parameters: vec![
                        ParameterDeclaration {
                            name: "x".to_owned(),
                            expression: Arc::new(Expression::Reference {
                                attribute: 0,
                            }),
                        },
                    ],
                },
                tuple: ConstrainedTuple {
                    ty_id: 0,
                    constraints: vec![],
                    alias: "smk".to_owned(),
                },
            },
            Predicate {
                ty: PredicateType::Event {
                    selection: EventSelection::Last,
                    parameters: vec![
                        ParameterDeclaration {
                            name: "y".to_owned(),
                            expression: Arc::new(Expression::Reference {
                                attribute: 1,
                            }),
                        },
                    ],
                    timing: Timing {
                        upper: 0,
                        bound: TimingBound::Within {
                            window: Duration::minutes(5),
                        },
                    },
                },
                tuple: ConstrainedTuple {
                    ty_id: 1,
                    constraints: vec![
                        Arc::new(Expression::BinaryOperation {
                            operator: BinaryOperator::Equal,
                            left: Box::new(Expression::Reference {
                                attribute: 0,
                            }),
                            right: Box::new(Expression::Parameter {
                                predicate: 0,
                                parameter: 0,
                            }),
                        }),
                        Arc::new(Expression::BinaryOperation {
                            operator: BinaryOperator::GreaterThan,
                            left: Box::new(Expression::Reference {
                                attribute: 1,
                            }),
                            right: Box::new(Expression::Immediate {
                                value: Value::Int(45),
                            }),
                        }),
                    ],
                    alias: "temp".to_owned(),
                },
            },
        ],
        filters: vec![],
        event_template: EventTemplate {
            ty_id: 2,
            attributes: vec![
                Expression::Parameter {
                    predicate: 0,
                    parameter: 0,
                },
                Expression::Parameter {
                    predicate: 1,
                    parameter: 0,
                },
            ],
        },
        consuming: vec![],
    });

    // We subscribe a listener to receive every event,
    // the `DebugListener` prints to stdout each event.
    engine.subscribe(Box::new(DebugListener));

    // Now we publish a sequence of Events
    //
    // The first is a temperature event,
    // not high enogh to satisfy the rule constraint.
    engine.publish(&Arc::new(Event {
        tuple: Tuple {
            ty_id: 1,
            data: vec![
                Value::Str("area_1".to_owned()),
                Value::Int(25),
            ],
        },
        time: UTC::now(),
    }));

    // Another temperature event that now satisfy the constraint.
    engine.publish(&Arc::new(Event {
        tuple: Tuple {
            ty_id: 1,
            data: vec![
                Value::Str("area_1".to_owned()),
                Value::Int(52),
            ],
        },
        time: UTC::now(),
    }));

    // Another temperature that satisfy the constraint,
    // but is on a different area from the previous ones.
    engine.publish(&Arc::new(Event {
        tuple: Tuple {
            ty_id: 1,
            data: vec![
                Value::Str("area_2".to_owned()),
                Value::Int(50),
            ],
        },
        time: UTC::now(),
    }));

    // Finally a smoke events arrives on area 1
    // and a fire event is triggered.
    engine.publish(&Arc::new(Event {
        tuple: Tuple {
            ty_id: 0,
            data: vec![Value::Str("area_1".to_owned())],
        },
        time: UTC::now(),
    }));
}
