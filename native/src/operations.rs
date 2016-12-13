use neon::vm::{Call, JsResult, Module};
use neon::js::JsString;
use neon::js::JsInteger;
use tesla::{Listener};

extern crate rustc_serialize;
use rustc_serialize::json::Json;
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
use conn_queues::{insert_queue, pop_queue, print_queue_status,remove_queue,init_queue};

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
pub struct QueueListener{
    // connID: String,
    connID: usize,
}

impl Listener for QueueListener {
    fn receive(&mut self, event: &Arc<Event>) {
        // println!("Queue Listener: {:?}", event);
        // println!("Queue Listener self.connID.to_owned() => {:?}", self.connID.to_owned());
        // insert_queue(self.connID.to_owned(), ( 15 as i32));

        // match Arc::try_unwrap(*event) {
        //     Ok(v) => insert_queue(self.connID.to_owned(), v),
        //     Err(e) => println!("Error"),
        //     // _ => println!("Error"),
        // }
        // insert_queue(self.connID.to_owned(), (*event).clone());
        insert_queue(self.connID.to_owned(), (*event).clone());
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
pub fn init_examples(){
    println!("Rust::initialize()");
    // let engine = singleton();
    // m_subscribe();
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

pub fn declareEvent(event_id: usize, event_name: &str, event_vector: Vec<AttributeDeclaration>){
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

pub fn defineRule(rule_predicate: Vec<Predicate>, r_e_template: EventTemplate){
    println!("Rust::defineRule(...)");
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
// pub fn subscribe() -> usize {
//     let s = singleton();
//     let mut engine = s.inner.lock().unwrap();
//     // engine.subscribe(Box::new(DebugListener))
//     engine.subscribe(Box::new(DebugListener2))
// }
pub fn subscribe() -> usize {
// pub fn subscribe(connID: String) -> usize {
    println!("Rust::subscribe(...)");
    let s = singleton();
    let mut engine = s.inner.lock().unwrap();

    // let queueL = Box::new(QueueListener{connID : String::from("asfa")});

    // engine.subscribe(Box::new(DebugListener2))



    // let conn_id = engine.last_id+(1 as usize);
    let conn_id = engine.get_last_id() + (1 as usize);

    // pop_queue(conn_id);
    init_queue(conn_id);

    engine.subscribe(Box::new(QueueListener{connID : conn_id}))
    // engine.subscribe(queueL)

    //Take connID and create a listener for that
}

// pub fn unsubscribe(id: &usize){
// pub fn unsubscribe(connID: String){
pub fn unsubscribe(connID: usize){
    println!("Rust::unsubscribe(...)");
    let s = singleton();
    let mut engine = s.inner.lock().unwrap();

    // engine.unsubscribe(id);
    engine.unsubscribe(&connID);

    //TODO: Remove queue
    remove_queue(connID);
}

// pub fn publish(type_id: usize, data_event : Vec<Value>){
pub fn publish(event: Json){

    let obj_event = event.as_object().unwrap();//BTreeMap<String, Json>

    // let time = obj_event.get("time").unwrap();//Json
    let tuple = obj_event.get("tuple").unwrap();//Json

    println!("Rust tuple: {}", tuple);

    let obj_tuple = tuple.as_object().unwrap();//BTreeMap<String, Json>
    println!("1");
    let ty_id = obj_tuple.get("ty_id").unwrap();//Json
    println!("2");
    let data = obj_tuple.get("data").unwrap();//Json
    println!("3");
    println!("Rust ty_id: {}", ty_id);
    let ty_id_u = ty_id.as_string().unwrap().parse::<usize>();// as usize;
    println!("4");
    let data_a = data.as_array().unwrap();//Vec<Json>
    println!("5");

    let mut vec_value: Vec<Value> = Vec::new();

    for data_e in data_a.iter(){//Json
        // println!("AAA: {:?}", data_e);
        // vec_value.push(Value)
        // Value::Str("area_1".to_owned())
        // Value::Int(52)

        println!("data_e: {:?}", data_e);
        println!("data_e.is_string: {:?}", data_e.is_string());
        println!("data_e.is_string: {:?}", data_e.is_number());

        if (data_e.as_string().unwrap().parse::<i32>().is_ok()){
            vec_value.push(Value::Int(data_e.as_string().unwrap().parse::<i32>().unwrap()));
        } else { //if (data_e.as_string().unwrap().parse::<usize>().is_ok())
            vec_value.push(Value::Str(String::from(data_e.as_string().unwrap())));
        }
    }




    // vec![
    //     Value::Str("area_1".to_owned()),
    //     Value::Int(52),
    // ]


    println!("Rust::publish(...)");

    println!("Rust::vec_value: {:?}", vec_value);
    println!("Rust::ty_id_u: {:?}", ty_id_u);


    let s = singleton();
    let mut engine = s.inner.lock().unwrap();
    engine.publish(&Arc::new(Event {
        tuple: Tuple {
            ty_id: ty_id_u.unwrap(),
            data: vec_value
            // vec![
            //     Value::Str("area_1".to_owned()),
            //     Value::Int(52),
            // ]
            ,
        },
        time: UTC::now(),
    }));
    /*let s = singleton();
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
    }));*/

    // m_publish(Event {
    //     tuple: Tuple {
    //         ty_id: type_id,
    //         data: data_event
    //         ,
    //     },
    //     time: UTC::now(),
    // });
}

// pub fn get_notification() -> Option<i32> {
// pub fn get_notification(connid: String) -> Option<Event> {
// pub fn get_notification(connid: String) -> Option<i32> {
// pub fn get_notification(connid: String) -> Option<Arc<Event>> {
pub fn get_notification(connid: usize) -> Option<Arc<Event>> {
    println!("Rust::get_notification(...)");
    // m_get_publish(0)
    pop_queue(connid)
}

pub fn status(){
    println!("Rust::status()");
    print_queue_status();
}
