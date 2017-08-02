#[macro_use]
extern crate neon;
extern crate rustc_serialize;
extern crate chrono;
extern crate num_cpus;
extern crate tesla;
extern crate trex;
extern crate uuid;

use rustc_serialize::json::Json;
use neon::vm::{Call, JsResult};
use neon::js::{JsInteger, JsString};
use uuid::Uuid;
use operations::{init_examples,declare_event, define_rule, subscribe, unsubscribe, publish, unknown_publish, get_notification, status, get_connection};
use conn_queues::write_status;

pub mod conn_queues;
pub mod operations;
pub mod json_conversions;

fn w_get_connection(call: Call) -> JsResult<JsString> {
    let scope = call.scope;
    let uuid = Uuid::new_v4();
    println!("Rust::getConnection: {}", uuid);

    get_connection(uuid);

    let result = write_status();

    if result.is_err() {
        print!("Error processing write_status()");
    }

    Ok(JsString::new(scope, &format!("{{\"result\" : \"ok\", \"value\" : \"{}\"}}",&(uuid.to_hyphenated_string())[..])[..]).unwrap())
}

fn w_init_examples(call: Call) -> JsResult<JsString> {
    let scope = call.scope;

    init_examples();

    if write_status().is_err() {
        print!("Error processing write_status()");
    }

    Ok(JsString::new(scope, "{\"result\" : \"ok\"}").unwrap())
}

fn w_clear_status(call: Call) -> JsResult<JsString> {
    let scope = call.scope;

    if write_status().is_err() {
        print!("Error processing write_status()");
    }

    Ok(JsString::new(scope, "{\"result\" : \"ok\"}").unwrap())
}

// Dummy function to test timing
fn w_measure_time(call: Call) -> JsResult<JsString> {
    let scope = call.scope;

    Ok(JsString::new(scope, "{\"result\" : \"ok\"}").unwrap())
}

fn w_declare_event(call: Call) -> JsResult<JsString> {
    let scope = call.scope;
    let str_event = try!(try!(call.arguments.require(scope, 0)).check::<JsString>()).value();

    let event_dec = Json::from_str(&str_event[..]).unwrap();//Json

    declare_event(event_dec);

    if write_status().is_err() {
        print!("Error processing write_status()");
    }

    Ok(JsString::new(scope, "{\"result\" : \"ok\"}").unwrap())
}

fn w_define_rule(call: Call) -> JsResult<JsString> {
    let scope = call.scope;
    let str_rule = try!(try!(call.arguments.require(scope, 0)).check::<JsString>()).value();

    let rule_def = Json::from_str(&str_rule[..]).unwrap();//Json

    define_rule(rule_def);

    if write_status().is_err() {
        print!("Error processing write_status()");
    }

    Ok(JsString::new(scope, "{\"result\" : \"ok\"}").unwrap())
}

fn w_subscribe(call: Call) -> JsResult<JsString> {
    let scope = call.scope;
    let conn_id = try!(try!(call.arguments.require(scope, 0)).check::<JsString>()).value();
    let event_type = try!(try!(call.arguments.require(scope, 1)).check::<JsInteger>()).value() as usize;

    let subs_return = subscribe(conn_id, event_type) as i32;

    if write_status().is_err() {
        print!("Error processing write_status()");
    }

    Ok(JsString::new(scope, &format!("{{\"result\" : \"ok\", \"value\" : {}}}",subs_return)[..]).unwrap())
}

fn w_unsubscribe(call: Call) -> JsResult<JsString> {
    let scope = call.scope;
    let conn_id = try!(try!(call.arguments.require(scope, 0)).check::<JsString>()).value();
    let subs_id = try!(try!(call.arguments.require(scope, 1)).check::<JsInteger>()).value() as usize;

    println!("{:?}", conn_id);
    println!("{:?}", subs_id);

    unsubscribe(subs_id);

    if write_status().is_err() {
        print!("Error processing write_status()");
    }

    Ok(JsString::new(scope, "{\"result\" : \"ok\"}").unwrap())
}

fn w_status(call: Call) -> JsResult<JsString> {
    let scope = call.scope;

    status();

    if write_status().is_err() {
        print!("Error processing write_status()");
    }

    Ok(JsString::new(scope, "{\"result\" : \"ok\"}").unwrap())
}

fn w_publish(call: Call) -> JsResult<JsString> {
    let scope = call.scope;

    let js_conn_id = try!(try!(call.arguments.require(scope, 0)).check::<JsInteger>()).value();
    let str_event = try!(try!(call.arguments.require(scope, 1)).check::<JsString>()).value();

    let conn_id = js_conn_id as usize;
    let event = Json::from_str(&str_event[..]).unwrap();//Json

    publish(conn_id, event);

    if write_status().is_err() {
        print!("Error processing write_status()");
    }

    Ok(JsString::new(scope, "{\"result\" : \"ok\"}").unwrap())
}

fn w_unknown_publish(call: Call) -> JsResult<JsString> {
    let scope = call.scope;
    let str_event = try!(try!(call.arguments.require(scope, 0)).check::<JsString>()).value();

    let event = Json::from_str(&str_event[..]).unwrap();//Json

    unknown_publish(event);

    if write_status().is_err() {
        print!("Error processing write_status()");
    }

    Ok(JsString::new(scope, "{\"result\" : \"ok\"}").unwrap())
}

fn w_get_notification(call: Call) -> JsResult<JsString> {
    println!("Rust::w_get_notification(...)");
    let scope = call.scope;
    let conn_id = try!(try!(call.arguments.require(scope, 0)).check::<JsString>()).value();
    let result = get_notification(conn_id);
    println!("Rust::result {:?}",result);

    if write_status().is_err() {
        print!("Error processing write_status()");
    }

    match result {
        Some(x) => Ok(JsString::new(scope, &format!("{:?}", x)[..]  ).unwrap()),
        _    => Ok(JsString::new(scope, "{\"result\" : \"Nothing to return\"}").unwrap()),
    }
}

register_module!(m, {
    m.export("init_examples", w_init_examples);
    m.export("clear_status", w_clear_status);
    m.export("get_connection", w_get_connection);
    m.export("declare_event", w_declare_event);
    m.export("define_rule", w_define_rule);
    m.export("subscribe", w_subscribe);
    m.export("unsubscribe", w_unsubscribe);
    m.export("status", w_status);
    m.export("get_notification", w_get_notification);
    m.export("publish", w_publish);
    m.export("unknown_publish", w_unknown_publish);
    m.export("measure_time", w_measure_time)
});
