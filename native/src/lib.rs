#[macro_use]
extern crate neon;

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
// use trex::TRex;
// use trex::listeners::DebugListener;
// use trex::stack::StackProvider;

// use std::io;
// use std::sync::{Mutex, Once, ONCE_INIT};
// use std::{mem, thread};

pub mod global_vector;
pub mod operations;

use operations::{initialize,declareEvent, defineRule, subscribe, unsubscribe, publish,get_notification};

///////////// WRAPPERS ////////////////////////////////////////////////////////
fn w_initialize(call: Call) -> JsResult<JsString> {
    let scope = call.scope;

    initialize();

    Ok(JsString::new(scope, "Ok").unwrap())
}

fn w_declareEvent(call: Call) -> JsResult<JsString> {
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


    Ok(JsString::new(scope, "Ok").unwrap())
}

fn w_defineRule(call: Call) -> JsResult<JsString> {
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

    Ok(JsString::new(scope, "Ok").unwrap())
}

fn w_subscribe(call: Call) -> JsResult<JsString> {
    let scope = call.scope;

    let subscriber_id = subscribe();

    Ok(JsString::new(scope, "Ok").unwrap())
}

fn w_unsubscribe(call: Call) -> JsResult<JsString> {
    let scope = call.scope;

    let subscriber_id:usize = 1;
    unsubscribe(&subscriber_id);

    Ok(JsString::new(scope, "Ok").unwrap())
}

fn w_publish(call: Call) -> JsResult<JsString> {
    let scope = call.scope;

    let attr1 = try!(try!(call.arguments.require(scope, 0)).check::<JsInteger>()).value();

    let type_id = attr1 as usize;
    let data_event = vec![Value::Str("area_1".to_owned())];
    publish(type_id, data_event);

    Ok(JsString::new(scope, "Ok").unwrap())
}

fn w_get_notification(call: Call) -> JsResult<JsString> {
    let scope = call.scope;

    let type_id = 0;
    let data_event = vec![Value::Str("area_1".to_owned())];
    let result = get_notification();

    match result {
        // The division was valid
        Some(x) => Ok(JsString::new(scope, &format!("{:?}", x)[..]  ).unwrap()),
        // The division was invalid
        // None    => println!("Cannot divide by 0"),
        _    => Ok(JsString::new(scope, "Nothing to Return").unwrap()),
    }


}

register_module!(m, {
    m.export("initialize", w_initialize);
    m.export("declareEvent", w_declareEvent);
    m.export("defineRule", w_defineRule);
    m.export("subscribe", w_subscribe);
    m.export("unsubscribe", w_unsubscribe);
    m.export("get_notification", w_get_notification);
    m.export("publish", w_publish)
});