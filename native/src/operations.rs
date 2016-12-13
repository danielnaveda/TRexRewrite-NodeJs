use tesla::{Listener};

extern crate rustc_serialize;
use rustc_serialize::json::Json;

extern crate chrono;
extern crate num_cpus;
extern crate tesla;
extern crate trex;

use chrono::{Duration, UTC};
use std::sync::Arc;
use tesla::{AttributeDeclaration, Engine, Event, EventTemplate, Rule, Tuple, TupleDeclaration,TupleType};
use tesla::expressions::{BasicType, BinaryOperator, Expression, Value};
use tesla::predicates::{ConstrainedTuple, EventSelection, ParameterDeclaration, Predicate,PredicateType, Timing, TimingBound};
use trex::TRex;
use trex::stack::StackProvider;

use std::sync::{Mutex, Once, ONCE_INIT};
use std::{mem};

use conn_queues::{insert_queue, pop_queue, print_queue_status,remove_queue,init_queue};

#[derive(Clone)]
struct SingletonReader { inner: Arc<Mutex<TRex>> }

fn singleton() -> SingletonReader {
    static mut SINGLETON: *const SingletonReader = 0 as *const SingletonReader;
    static ONCE: Once = ONCE_INIT;

    unsafe {
        ONCE.call_once(|| {
            let provider = Box::new(StackProvider);
            let singleton = SingletonReader {
                inner: Arc::new(Mutex::new(TRex::new(num_cpus::get(), vec![provider])))
            };

            SINGLETON = mem::transmute(Box::new(singleton));
        });

        (*SINGLETON).clone()
    }
}

#[derive(Clone, Debug)]
pub struct QueueListener{
    conn_id: usize,
}

impl Listener for QueueListener {
    fn receive(&mut self, event: &Arc<Event>) {
        insert_queue(self.conn_id.to_owned(), (*event).clone());
    }
}

pub fn init_examples(){
    println!("Rust::initialize()");
    let s = singleton();
    let mut engine = s.inner.lock().unwrap();

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
}

pub fn declare_event(event_id: usize, event_name: &str, event_vector: Vec<AttributeDeclaration>){
    println!("Rust::declareEvent(...)");
    let s = singleton();
    let mut engine = s.inner.lock().unwrap();
    engine.declare(TupleDeclaration {
        ty: TupleType::Event,
        id: event_id,
        name: event_name.to_owned(),
        attributes: event_vector,
    });
}

pub fn define_rule(rule_predicate: Vec<Predicate>, r_e_template: EventTemplate){
    println!("Rust::defineRule(...)");
    let s = singleton();
    let mut engine = s.inner.lock().unwrap();
    engine.define(Rule {
        predicates: rule_predicate,
        filters: vec![],
        event_template: r_e_template,
        consuming: vec![],
    });
}

// pub fn subscribe(connID: String) -> usize {
pub fn subscribe() -> usize {
    println!("Rust::subscribe(...)");
    let s = singleton();
    let mut engine = s.inner.lock().unwrap();

    let conn_id = engine.get_last_id() + (1 as usize);

    init_queue(conn_id);

    engine.subscribe(Box::new(QueueListener{conn_id : conn_id}))
}

pub fn unsubscribe(conn_id: usize){
    println!("Rust::unsubscribe(...)");
    let s = singleton();
    let mut engine = s.inner.lock().unwrap();

    engine.unsubscribe(&conn_id);

    remove_queue(conn_id);
}

pub fn publish(event: Json){
    let obj_event = event.as_object().unwrap();//BTreeMap<String, Json>
    let tuple = obj_event.get("tuple").unwrap();//Json
    let obj_tuple = tuple.as_object().unwrap();//BTreeMap<String, Json>
    let ty_id = obj_tuple.get("ty_id").unwrap();//Json
    let data = obj_tuple.get("data").unwrap();//Json
    let ty_id_u = ty_id.as_string().unwrap().parse::<usize>();// as usize;
    let data_a = data.as_array().unwrap();//Vec<Json>

    let mut vec_value: Vec<Value> = Vec::new();

    for data_e in data_a.iter(){//Json
        if data_e.as_string().unwrap().parse::<i32>().is_ok() {
            vec_value.push(Value::Int(data_e.as_string().unwrap().parse::<i32>().unwrap()));
        } else {
            vec_value.push(Value::Str(String::from(data_e.as_string().unwrap())));
        }
    }

    let s = singleton();
    let mut engine = s.inner.lock().unwrap();
    engine.publish(&Arc::new(Event {
        tuple: Tuple {
            ty_id: ty_id_u.unwrap(),
            data: vec_value,
        },
        time: UTC::now(),
    }));
}

pub fn get_notification(conn_id: usize) -> Option<Arc<Event>> {
    println!("Rust::get_notification(...)");
    pop_queue(conn_id)
}

pub fn status(){
    println!("Rust::status()");
    print_queue_status();
}
