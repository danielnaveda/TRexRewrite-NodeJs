use neon::vm::{Call, JsResult, Module};
use neon::js::JsString;
use neon::js::JsInteger;
use tesla::{Listener};
///////////////////////////////////////////

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//     }
// }
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

use global_vector::{m_subscribe, m_publish, m_get_publish};

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

#[derive(Clone, Debug)]
pub struct DebugListener2;
impl Listener for DebugListener2 {
    fn receive(&mut self, event: &Arc<Event>) {
        println!("{:?}", event);
        // TODO
    }
}

#[derive(Clone, Debug)]
pub struct queue_struct;
impl Listener for queue_struct {
    fn receive(&mut self, event: &Arc<Event>) {
        // println!("{:?}", event);
        // TODO

        m_publish(

            Event {
                tuple: Tuple {
                    ty_id: 5,
                    data: vec![Value::Str("area_1".to_owned())],
                },
                time: UTC::now(),
            }
        );

        // vectorsingleton();
    }
}

///////////////
pub fn initialize(){
    let engine = singleton();
    m_subscribe();
}

pub fn declareEvent(event_id: usize, event_name: &str, event_vector: Vec<AttributeDeclaration>){
    let s = singleton();
    let mut engine = s.inner.lock().unwrap();
    engine.declare(TupleDeclaration {
        ty: TupleType::Event,
        id: event_id,
        name: event_name.to_owned(),
        attributes: event_vector,
    });
}

pub fn defineRule(rule_predicate: Vec<Predicate>, r_e_template: EventTemplate){
    let s = singleton();
    let mut engine = s.inner.lock().unwrap();
    engine.define(Rule {
        predicates: rule_predicate
        // vec![
        //     Predicate {
        //         ty: PredicateType::Trigger {
        //             parameters: vec![
        //                 ParameterDeclaration {
        //                     name: "x".to_owned(),
        //                     expression: Arc::new(Expression::Reference {
        //                         attribute: 0,
        //                     }),
        //                 },
        //             ],
        //         },
        //         tuple: ConstrainedTuple {
        //             ty_id: 0,
        //             constraints: vec![],
        //             alias: "smk".to_owned(),
        //         },
        //     },
        //     Predicate {
        //         ty: PredicateType::Event {
        //             selection: EventSelection::Last,
        //             parameters: vec![
        //                 ParameterDeclaration {
        //                     name: "y".to_owned(),
        //                     expression: Arc::new(Expression::Reference {
        //                         attribute: 1,
        //                     }),
        //                 },
        //             ],
        //             timing: Timing {
        //                 upper: 0,
        //                 bound: TimingBound::Within {
        //                     window: Duration::minutes(5),
        //                 },
        //             },
        //         },
        //         tuple: ConstrainedTuple {
        //             ty_id: 1,
        //             constraints: vec![
        //                 Arc::new(Expression::BinaryOperation {
        //                     operator: BinaryOperator::Equal,
        //                     left: Box::new(Expression::Reference {
        //                         attribute: 0,
        //                     }),
        //                     right: Box::new(Expression::Parameter {
        //                         predicate: 0,
        //                         parameter: 0,
        //                     }),
        //                 }),
        //                 Arc::new(Expression::BinaryOperation {
        //                     operator: BinaryOperator::GreaterThan,
        //                     left: Box::new(Expression::Reference {
        //                         attribute: 1,
        //                     }),
        //                     right: Box::new(Expression::Immediate {
        //                         value: Value::Int(45),
        //                     }),
        //                 }),
        //             ],
        //             alias: "temp".to_owned(),
        //         },
        //     },
        // ]
        ,
        filters: vec![],
        event_template: r_e_template
        // EventTemplate {
        //     ty_id: 2,
        //     attributes: vec![
        //         Expression::Parameter {
        //             predicate: 0,
        //             parameter: 0,
        //         },
        //         Expression::Parameter {
        //             predicate: 1,
        //             parameter: 0,
        //         },
        //     ],
        // }
        ,
        consuming: vec![],
    });
}

//TODO: implement a listener
pub fn subscribe() -> usize {
    let s = singleton();
    let mut engine = s.inner.lock().unwrap();
    // engine.subscribe(Box::new(DebugListener))
    engine.subscribe(Box::new(DebugListener2))
}

pub fn unsubscribe(id: &usize){
    let s = singleton();
    let mut engine = s.inner.lock().unwrap();
    engine.unsubscribe(id);
}

pub fn publish(type_id: usize, data_event : Vec<Value>){
    let s = singleton();
    let mut engine = s.inner.lock().unwrap();
    engine.publish(&Arc::new(Event {
        tuple: Tuple {
            ty_id: type_id,
            data: data_event.clone()
            // vec![
            //     Value::Str("area_1".to_owned()),
            //     Value::Int(52),
            // ]
            ,
        },
        time: UTC::now(),
    }));
    m_publish(Event {
        tuple: Tuple {
            ty_id: type_id,
            data: data_event
            ,
        },
        time: UTC::now(),
    });
}

// pub fn get_notification() -> Option<i32> {
pub fn get_notification() -> Option<Event> {
    println!("Rust: get_notification()");
    m_get_publish(0)
}
