#[macro_use]
extern crate neon;

extern crate rustc_serialize;
use rustc_serialize::json::Json;

use neon::vm::{Call, JsResult};
use neon::js::{JsInteger, JsString};

extern crate chrono;
extern crate num_cpus;
extern crate tesla;
extern crate trex;

extern crate uuid;
use uuid::Uuid;

use chrono::{Duration};
use std::sync::Arc;
use tesla::{AttributeDeclaration, EventTemplate};
use tesla::expressions::{BasicType, BinaryOperator, Expression, Value};
use tesla::predicates::{ConstrainedTuple, EventSelection, ParameterDeclaration, Predicate,PredicateType, Timing, TimingBound};

pub mod conn_queues;
pub mod operations;
pub mod json_conversions;

use operations::{init_examples,declare_event, define_rule, subscribe, unsubscribe, publish, unknown_publish, get_notification, status, get_connection};
use conn_queues::write_status;

fn w_get_connection(call: Call) -> JsResult<JsString> {
    let scope = call.scope;
    let uuid = Uuid::new_v4();
    println!("Rust::getConnection: {}", uuid);

    get_connection(uuid);

    write_status();

    Ok(JsString::new(scope, &format!("{{\"result\" : \"ok\", \"value\" : \"{}\"}}",&(uuid.to_hyphenated_string())[..])[..]).unwrap())
}

fn w_init_examples(call: Call) -> JsResult<JsString> {
    let scope = call.scope;

    init_examples();
    write_status();

    Ok(JsString::new(scope, "{\"result\" : \"ok\"}").unwrap())
}

// fn w_declare_event(call: Call) -> JsResult<JsString> {
//     let scope = call.scope;
//
//     let attr1 = try!(try!(call.arguments.require(scope, 0)).check::<JsInteger>()).value();
//     let attr2 = try!(try!(call.arguments.require(scope, 1)).check::<JsString>()).value();
//
//     let event_id = attr1 as usize;
//     let event_name =  &attr2[..];
//     let event_vector = vec![
//                                 AttributeDeclaration {
//                                     name: "area".to_owned(),
//                                     ty: BasicType::Str,
//                                 },
//                             ];
//     declare_event(event_id, event_name, event_vector);
//     write_status();
//     // Ok(JsString::new(scope, "Ok").unwrap())
//     Ok(JsString::new(scope, "{\"result\" : \"ok\"}").unwrap())
// }
fn w_declare_event(call: Call) -> JsResult<JsString> {
    let scope = call.scope;
    let str_event = try!(try!(call.arguments.require(scope, 0)).check::<JsString>()).value();

    let event_dec = Json::from_str(&str_event[..]).unwrap();//Json

    declare_event(event_dec);
    write_status();
    Ok(JsString::new(scope, "{\"result\" : \"ok\"}").unwrap())
}

fn w_define_rule(call: Call) -> JsResult<JsString> {
    let scope = call.scope;
    let str_rule = try!(try!(call.arguments.require(scope, 0)).check::<JsString>()).value();

    let rule_def = Json::from_str(&str_rule[..]).unwrap();//Json

    define_rule(rule_def);
    write_status();
    Ok(JsString::new(scope, "{\"result\" : \"ok\"}").unwrap())
}
// fn w_define_rule(call: Call) -> JsResult<JsString> {
//     let scope = call.scope;
//     let rule_predicate = vec![
//         Predicate {
//             ty: PredicateType::Trigger {
//                 parameters: vec![
//                     ParameterDeclaration {
//                         name: "x".to_owned(),
//                         expression: Arc::new(Expression::Reference {
//                             attribute: 0,
//                         }),
//                     },
//                 ],
//             },
//             tuple: ConstrainedTuple {
//                 ty_id: 0,
//                 constraints: vec![],
//                 alias: "smk".to_owned(),
//             },
//         },
//         Predicate {
//             ty: PredicateType::Event {
//                 selection: EventSelection::Last,
//                 parameters: vec![
//                     ParameterDeclaration {
//                         name: "y".to_owned(),
//                         expression: Arc::new(Expression::Reference {
//                             attribute: 1,
//                         }),
//                     },
//                 ],
//                 timing: Timing {
//                     upper: 0,
//                     bound: TimingBound::Within {
//                         window: Duration::minutes(5),
//                     },
//                 },
//             },
//             tuple: ConstrainedTuple {
//                 ty_id: 1,
//                 constraints: vec![
//                     Arc::new(Expression::BinaryOperation {
//                         operator: BinaryOperator::Equal,
//                         left: Box::new(Expression::Reference {
//                             attribute: 0,
//                         }),
//                         right: Box::new(Expression::Parameter {
//                             predicate: 0,
//                             parameter: 0,
//                         }),
//                     }),
//                     Arc::new(Expression::BinaryOperation {
//                         operator: BinaryOperator::GreaterThan,
//                         left: Box::new(Expression::Reference {
//                             attribute: 1,
//                         }),
//                         right: Box::new(Expression::Immediate {
//                             value: Value::Int(45),
//                         }),
//                     }),
//                 ],
//                 alias: "temp".to_owned(),
//             },
//         },];
//     let r_e_template = EventTemplate {
//         ty_id: 2,
//         attributes: vec![
//             Expression::Parameter {
//                 predicate: 0,
//                 parameter: 0,
//             },
//             Expression::Parameter {
//                 predicate: 1,
//                 parameter: 0,
//             },
//         ],};
//     define_rule(rule_predicate, r_e_template);
//     write_status();
//     // Ok(JsString::new(scope, "Ok").unwrap())
//     Ok(JsString::new(scope, "{\"result\" : \"ok\"}").unwrap())
// }

fn w_subscribe(call: Call) -> JsResult<JsString> {
    let scope = call.scope;
    let conn_id = try!(try!(call.arguments.require(scope, 0)).check::<JsString>()).value();
    let event_type = try!(try!(call.arguments.require(scope, 1)).check::<JsInteger>()).value() as usize;

    let subs_return = subscribe(conn_id, event_type) as i32;

    write_status();
    Ok(JsString::new(scope, &format!("{{\"result\" : \"ok\", \"value\" : {}}}",subs_return)[..]).unwrap())
}

fn w_unsubscribe(call: Call) -> JsResult<JsString> {
    let scope = call.scope;
    // let conn_id = try!(try!(call.arguments.require(scope, 0)).check::<JsInteger>()).value();
    let conn_id = try!(try!(call.arguments.require(scope, 0)).check::<JsString>()).value();
    let subs_id = try!(try!(call.arguments.require(scope, 1)).check::<JsInteger>()).value() as usize;


    println!("{:?}", conn_id);
    println!("{:?}", subs_id);

    // unsubscribe((conn_id as usize));
    unsubscribe(subs_id);

    write_status();
    // Ok(JsString::new(scope, "Ok").unwrap())
    Ok(JsString::new(scope, "{\"result\" : \"ok\"}").unwrap())
}

fn w_status(call: Call) -> JsResult<JsString> {
    let scope = call.scope;

    status();
    write_status();
    // Ok(JsString::new(scope, "Ok").unwrap())
    Ok(JsString::new(scope, "{\"result\" : \"ok\"}").unwrap())
}

fn w_publish(call: Call) -> JsResult<JsString> {
    let scope = call.scope;

    let js_conn_id = try!(try!(call.arguments.require(scope, 0)).check::<JsInteger>()).value();
    let str_event = try!(try!(call.arguments.require(scope, 1)).check::<JsString>()).value();

    let conn_id = js_conn_id as usize;
    let event = Json::from_str(&str_event[..]).unwrap();//Json

    publish(conn_id, event);
    write_status();
    Ok(JsString::new(scope, "{\"result\" : \"ok\"}").unwrap())
}

fn w_unknown_publish(call: Call) -> JsResult<JsString> {
    let scope = call.scope;
    let str_event = try!(try!(call.arguments.require(scope, 0)).check::<JsString>()).value();

    let event = Json::from_str(&str_event[..]).unwrap();//Json

    unknown_publish(event);
    write_status();
    Ok(JsString::new(scope, "{\"result\" : \"ok\"}").unwrap())
}

fn w_get_notification(call: Call) -> JsResult<JsString> {
    println!("Rust::w_get_notification(...)");
    let scope = call.scope;
    // let conn_id = try!(try!(call.arguments.require(scope, 0)).check::<JsInteger>()).value();
    let conn_id = try!(try!(call.arguments.require(scope, 0)).check::<JsString>()).value();
    // let result = get_notification(conn_id as usize);
    let result = get_notification(conn_id);
    println!("Rust::result {:?}",result);

    write_status();
    match result {
        Some(x) => Ok(JsString::new(scope, &format!("{:?}", x)[..]  ).unwrap()),
        // _    => Ok(JsString::new(scope, "Nothing to Return").unwrap()),
        _    => Ok(JsString::new(scope, "{\"result\" : \"Nothing to return\"}").unwrap()),
    }
}

register_module!(m, {
    m.export("init_examples", w_init_examples);
    m.export("get_connection", w_get_connection);
    m.export("declare_event", w_declare_event);
    m.export("define_rule", w_define_rule);
    m.export("subscribe", w_subscribe);
    m.export("unsubscribe", w_unsubscribe);
    m.export("status", w_status);
    m.export("get_notification", w_get_notification);
    m.export("publish", w_publish);
    m.export("unknown_publish", w_unknown_publish)
});
