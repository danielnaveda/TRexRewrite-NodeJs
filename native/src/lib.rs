#[macro_use]
extern crate neon;

extern crate rustc_serialize;
use rustc_serialize::json::Json;

use neon::vm::{Call, JsResult, Module};
use neon::js::{JsInteger, JsString, JsObject};
use tesla::{Listener};
use neon::mem::Handle;
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

extern crate uuid;
use uuid::Uuid;

use chrono::{Duration, UTC};
use std::sync::Arc;
use tesla::{AttributeDeclaration, Engine, Event, EventTemplate, Rule, Tuple, TupleDeclaration,
            TupleType};
use tesla::expressions::{BasicType, BinaryOperator, Expression, Value};
use tesla::predicates::{ConstrainedTuple, EventSelection, ParameterDeclaration, Predicate,
                        PredicateType, Timing, TimingBound};
// use trex::TRex;
// use trex::listeners::DebugListener;
// use trex::stack::StackProvider;

// use std::io;
// use std::sync::{Mutex, Once, ONCE_INIT};
// use std::{mem, thread};

pub mod global_vector;
pub mod conn_queues;
pub mod operations;

use operations::{init_examples,declareEvent, defineRule, subscribe, unsubscribe, publish,get_notification,status};
use conn_queues::write_status;
///////////// WRAPPERS ////////////////////////////////////////////////////////
fn w_getConnection(call: Call) -> JsResult<JsString> {
    // println!("w_getConnection");
    let scope = call.scope;
    let uuid = Uuid::new_v4();
    println!("Rust::getConnection: {}", uuid);
    // Ok(JsString::new(scope, "5156165516548").unwrap())
    Ok(JsString::new(scope, &(uuid.to_hyphenated_string())[..]).unwrap())
}

fn w_init_examples(call: Call) -> JsResult<JsString> {
    // println!("w_initialize");
    let scope = call.scope;

    init_examples();
    write_status();
    Ok(JsString::new(scope, "Ok").unwrap())
}

fn w_declareEvent(call: Call) -> JsResult<JsString> {
    // println!("w_declareEvent");
    let scope = call.scope;

    let attr1 = try!(try!(call.arguments.require(scope, 0)).check::<JsInteger>()).value();
    let attr2 = try!(try!(call.arguments.require(scope, 1)).check::<JsString>()).value();


    // let event_id = 0;
    // let event_name = "smoke";
    let event_id = attr1 as usize;
    let event_name =  &attr2[..];
    // println!("{}",event_name);
    let event_vector = vec![
                                AttributeDeclaration {
                                    name: "area".to_owned(),
                                    ty: BasicType::Str,
                                },
                            ];
    declareEvent(event_id, event_name, event_vector);

    write_status();
    Ok(JsString::new(scope, "Ok").unwrap())
}

fn w_defineRule(call: Call) -> JsResult<JsString> {
    // println!("w_defineRule");
    let scope = call.scope;

    let rule_predicate = vec![
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
        },];
    let r_e_template = EventTemplate {
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
        ],};
    defineRule(rule_predicate, r_e_template);

    write_status();
    Ok(JsString::new(scope, "Ok").unwrap())
}

fn w_subscribe(call: Call) -> JsResult<JsString> {
    // println!("w_subscribe");
    let scope = call.scope;

    // let connID = try!(try!(call.arguments.require(scope, 0)).check::<JsString>()).value();
    // println!("{}",connID);
    // let subscriber_id = subscribe();
    // subscribe(connID);
    let subs_return = subscribe() as i32;

    write_status();
    // Ok(JsString::new(scope, "Ok").unwrap())
    Ok(JsString::new(scope, &format!("{}",subs_return)[..]).unwrap())
}

fn w_unsubscribe(call: Call) -> JsResult<JsString> {
    // println!("w_unsubscribe");
    let scope = call.scope;
    // let connID = try!(try!(call.arguments.require(scope, 0)).check::<JsString>()).value();
    let connID = try!(try!(call.arguments.require(scope, 0)).check::<JsInteger>()).value();
    // let subscriber_id:usize = 1;
    // unsubscribe(&subscriber_id);
    unsubscribe((connID as usize));

    write_status();
    Ok(JsString::new(scope, "Ok").unwrap())
}

fn w_status(call: Call) -> JsResult<JsString> {
    // println!("w_status");
    let scope = call.scope;
    status();
    write_status();
    Ok(JsString::new(scope, "Ok").unwrap())
}

fn w_publish(call: Call) -> JsResult<JsString> {
    // println!("w_publish");
    let scope = call.scope;

    // let connID = try!(try!(call.arguments.require(scope, 0)).check::<JsString>()).value();
    // let value = try!(try!(call.arguments.require(scope, 1)).check::<JsInteger>()).value();
    let connID = try!(try!(call.arguments.require(scope, 0)).check::<JsString>()).value();
    let type_input = try!(try!(call.arguments.require(scope, 1)).check::<JsInteger>()).value();
    let area = try!(try!(call.arguments.require(scope, 2)).check::<JsString>()).value();

    // let type_id = value as usize;
    let type_id = type_input as usize;
    // let data_event = vec![Value::Str("area_1".to_owned())];
    let data_event = vec![Value::Str(area)];
    // publish(type_id, data_event);

    write_status();
    Ok(JsString::new(scope, "Ok").unwrap())
}

fn w_unknown_publish(call: Call) -> JsResult<JsString> {
    let scope = call.scope;
    let str_event = try!(try!(call.arguments.require(scope, 0)).check::<JsString>()).value();

    let event = Json::from_str(&str_event[..]).unwrap();//Json

    publish(event);

    write_status();

    Ok(JsString::new(scope, "Ok").unwrap())


    // println!("Rust string: {}", str_event);
    //
    // let event = Json::from_str(&str_event[..]).unwrap();//Json
    //
    // println!("Rust json::from_str: {}", event);
    //
    // let obj_event = event.as_object().unwrap();//BTreeMap<String, Json>
    //
    // let time = obj_event.get("time").unwrap();//Json
    // println!("Rust time: {}", time);
    //
    // let tuple = obj_event.get("tuple").unwrap();//Json
    // println!("Rust tuple: {}", tuple);
    //
    // let obj_tuple = tuple.as_object().unwrap();//BTreeMap<String, Json>
    //
    // for x in obj_tuple.iter(){
    //     println!("Rust data: {:?}", x);
    // }
    //
    // let obj_tuple3 = tuple.as_object().unwrap();//BTreeMap<String, Json>
    // let data = obj_tuple3.get("data").unwrap();//Json
    //
    // let data2 = data.as_array().unwrap();//Vec<Json>
    // for x in data2.iter(){
    //     println!("AAA: {:?}", x);
    // }
    //
    // let data = obj_tuple.get("data").unwrap();//Json
    // let obj_tuple = tuple.as_object().unwrap();//Vec<Json>
    //
    // write_status();
    // Ok(JsString::new(scope, "Ok").unwrap())
}

fn w_get_notification(call: Call) -> JsResult<JsString> {
    let scope = call.scope;

    // let connID = try!(try!(call.arguments.require(scope, 0)).check::<JsString>()).value();
    let connID = try!(try!(call.arguments.require(scope, 0)).check::<JsInteger>()).value();

    // let type_id = 0;
    // let data_event = vec![Value::Str("area_1".to_owned())];
    let result = get_notification(connID as usize);
    write_status();
    match result {
        // The division was valid
        Some(x) => Ok(JsString::new(scope, &format!("{:?}", x)[..]  ).unwrap()),
        // The division was invalid
        // None    => println!("Cannot divide by 0"),
        _    => Ok(JsString::new(scope, "Nothing to Return").unwrap()),
    }


}

register_module!(m, {
    m.export("init_examples", w_init_examples);
    m.export("getConnection", w_getConnection);
    m.export("declareEvent", w_declareEvent);
    m.export("defineRule", w_defineRule);
    m.export("subscribe", w_subscribe);
    m.export("unsubscribe", w_unsubscribe);
    m.export("status", w_status);
    // m.export("get_notification", w_get_notification);
    m.export("getNotification", w_get_notification);
    m.export("publish", w_publish);
    m.export("unknown_publish", w_unknown_publish)
});
