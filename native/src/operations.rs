extern crate rustc_serialize;
extern crate chrono;
extern crate num_cpus;
extern crate tesla;
extern crate trex;
extern crate uuid;

use tesla::{Listener};
use rustc_serialize::json::Json;
use uuid::Uuid;
use chrono::{Duration};
use std::sync::Arc;
use tesla::{AttributeDeclaration, Engine, Event, EventTemplate, Rule, TupleDeclaration,TupleType};
use tesla::expressions::{BasicType, BinaryOperator, Expression, Value};
use tesla::predicates::{ConstrainedTuple, EventSelection, ParameterDeclaration, Predicate,PredicateType, Timing, TimingBound};
use trex::TRex;
use trex::stack::StackProvider;
use std::sync::{Mutex, Once, ONCE_INIT};
use std::{mem};
use conn_queues::{insert_queue, pop_queue, print_queue_status,init_queue};
use json_conversions::{JsonConversion};

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
    conn_id: String,
}

impl Listener for QueueListener {
    fn receive(&mut self, event: &Arc<Event>) {
        insert_queue(self.conn_id.to_owned(), (*event).clone());
    }
}

pub fn get_connection(uuid: Uuid) {
    init_queue(uuid.to_string());
}



pub fn init_examples(){
    println!("Rust::initialize()");
    let s = singleton();
    let mut engine = s.inner.lock().unwrap();

    // `declare smoke(area: string) with id 0`
    let declare1 = TupleDeclaration {
        ty: TupleType::Event,
        id: 0,
        name: "smoke".to_owned(),
        attributes: vec![
                AttributeDeclaration {
                    name: "area".to_owned(),
                    ty: BasicType::Str,
                },
            ],
    };
    println!("\ndeclare1 {:?}\n", declare1);
    engine.declare(declare1);

    let declare2 = TupleDeclaration {
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
    };
    println!("\ndeclare2 {:?}\n", declare2);
    engine.declare(declare2);

    let declare3 = TupleDeclaration {
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
    };
    println!("\ndeclare3 {:?}\n", declare3);
    engine.declare(declare3);

    let rule1 = Rule {
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
    };
    println!("\nrule1 {:?}\n", rule1);
    engine.define(rule1);

    println!("Smoke ID: {}",engine.tuple_id("smoke").unwrap());
    println!("Temperature.value ID: {}",engine.tupleattr_id("temperature","value").unwrap());
    println!("Fire ID: {}",engine.tuple_id("fire").unwrap());
    println!("Fire.area ID: {}",engine.tupleattr_id("fire","area").unwrap());
}

pub fn declare_event(event: Json){
    println!("Rust::declareEvent(...)");
    let s = singleton();
    let mut engine = s.inner.lock().unwrap();

    let event_struct = TupleDeclaration::from_json(event);
    println!("Rust::defineRule::event_struct: {:?}\n",event_struct);
    engine.declare(event_struct);
}

pub fn define_rule(rule: Json){
    println!("Rust::defineRule(...)");

    let rule_struct = Rule::from_json(rule);
    println!("Rust::defineRule::rule_struct: {:?}\n",rule_struct);

    let s = singleton();
    let mut engine = s.inner.lock().unwrap();
    
    engine.define(rule_struct);
}

pub fn subscribe(conn_id: String, event_type: usize) -> usize {
    println!("Rust::subscribe(...)");
    let s = singleton();
    let mut engine = s.inner.lock().unwrap();

    engine.subscribe(Box::new(QueueListener{conn_id : conn_id}))
}

pub fn unsubscribe(conn_id: usize){
    println!("Rust::unsubscribe(...)");
    let s = singleton();
    let mut engine = s.inner.lock().unwrap();

    engine.unsubscribe(&conn_id);

    // remove_queue(conn_id);
}

pub fn publish(conn_id: usize ,event: Json){
    // TODO: write algorithm for publish (similar to unknown_publish() but taking conn_id into account)
    unknown_publish(event);
}

pub fn unknown_publish(event: Json){
    let s = singleton();
    let mut engine = s.inner.lock().unwrap();

    // engine.publish(&Arc::new(json_to_event(event)));
    engine.publish(&Arc::new(Event::from_json(event)));
}

pub fn get_notification(conn_id: String) -> Option<Arc<Event>> {
    println!("Rust::get_notification(...)");
    pop_queue(conn_id)
}

// fn tuple_id(&mut self, name: &str) -> Option<usize>;
pub fn get_tuple_id(name: &str) -> Option<usize> {
    let s = singleton();
    let mut engine = s.inner.lock().unwrap();

    engine.tuple_id(name)
}

// fn tupleattr_id(&mut self, tuple_name: &str, attr_name: &str) -> Option<usize>;
pub fn get_tupleattr_id(tuple_name: &str, attr_name: &str) -> Option<usize> {
    let s = singleton();
    let mut engine = s.inner.lock().unwrap();

    engine.tupleattr_id(tuple_name, attr_name)
}



pub fn status(){
    println!("Rust::status()");
    print_queue_status();
}
