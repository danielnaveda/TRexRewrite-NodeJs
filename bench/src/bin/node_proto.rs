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

use std::io;
use std::sync::{Mutex, Once, ONCE_INIT};
use std::{mem, thread};

#[derive(Clone)]
struct SingletonReader {
    // Since we will be used in many threads, we need to protect
    // concurrent access
    inner: Arc<Mutex<TRex>>
}

fn singleton() -> SingletonReader {
    // Initialize it to a null value
    static mut SINGLETON: *const SingletonReader = 0 as *const SingletonReader;
    static ONCE: Once = ONCE_INIT;

    unsafe {
        ONCE.call_once(|| {
            // Make it
            let provider = Box::new(StackProvider);
            let singleton = SingletonReader {
                // inner: Arc::new(Mutex::new(0))
                inner: Arc::new(Mutex::new(TRex::new(num_cpus::get(), vec![provider])))
            };

            // Put it in the heap so it can outlive this call
            SINGLETON = mem::transmute(Box::new(singleton));
        });

        // Now we give out a copy of the data that is safe to use concurrently.
        (*SINGLETON).clone()
    }
}

fn function_main() {
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

fn function_main_2(){
    let s = singleton();
    let mut engine = s.inner.lock().unwrap();

        // let mut engine = TRex::new(num_cpus::get(), vec![provider]);

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
        // engine.publish(&Arc::new(Event {
        //     tuple: Tuple {
        //         ty_id: 1,
        //         data: vec![
        //             Value::Str("area_1".to_owned()),
        //             Value::Int(25),
        //         ],
        //     },
        //     time: UTC::now(),
        // }));
        //
        // // Another temperature event that now satisfy the constraint.
        // engine.publish(&Arc::new(Event {
        //     tuple: Tuple {
        //         ty_id: 1,
        //         data: vec![
        //             Value::Str("area_1".to_owned()),
        //             Value::Int(52),
        //         ],
        //     },
        //     time: UTC::now(),
        // }));
        //
        // // Another temperature that satisfy the constraint,
        // // but is on a different area from the previous ones.
        // engine.publish(&Arc::new(Event {
        //     tuple: Tuple {
        //         ty_id: 1,
        //         data: vec![
        //             Value::Str("area_2".to_owned()),
        //             Value::Int(50),
        //         ],
        //     },
        //     time: UTC::now(),
        // }));
        //
        // // Finally a smoke events arrives on area 1
        // // and a fire event is triggered.
        // engine.publish(&Arc::new(Event {
        //     tuple: Tuple {
        //         ty_id: 0,
        //         data: vec![Value::Str("area_1".to_owned())],
        //     },
        //     time: UTC::now(),
        // }));
}

fn initialization(){
    // let provider = Box::new(StackProvider);
    // static mut engine2:  TRex = TRex::new(1, vec![Box::new(StackProvider)]);

    // let mut engine = TRex::new(num_cpus::get(), vec![provider]);
}

fn declare(){}
fn define(){}
fn subscribe(){}
fn publish_smoke(){
    let s = singleton();
    let mut engine = s.inner.lock().unwrap();
    engine.publish(&Arc::new(Event {
        tuple: Tuple {
            ty_id: 0,
            data: vec![Value::Str("area_1".to_owned())],
        },
        time: UTC::now(),
    }));
}
fn publish_temp(){
    let s = singleton();
    let mut engine = s.inner.lock().unwrap();
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
}

fn publish(){}


fn main() {

    // let s = singleton();
    // let mut data = s.inner.lock().unwrap();
    // // *data = i as u8;
    // data.declare(TupleDeclaration {
    //     ty: TupleType::Event,
    //     id: 0,
    //     name: "smoke".to_owned(),
    //     attributes: vec![
    //             AttributeDeclaration {
    //                 name: "area".to_owned(),
    //                 ty: BasicType::Str,
    //             },
    //         ],
    // });
// return;
    println!("Command Line Interface for TRexRewrite:");

    // initialization();
    let mut input = String::new();

    loop {
        input=String::from("");
        let mut n = io::stdin().read_line(&mut input);

        match input.trim() {
            "1" => {
                // println!("{}", input);
                initialization();
            }
            "2" => {
                // println!("{}", input);
                // function_main();
                function_main_2();
            }
            "3" => {
                publish_temp();
            }
            "4" => {
                publish_smoke();
            }
            "0" => {
                break;
            }
            _ => println!("nothing"),
        }

    }
}
