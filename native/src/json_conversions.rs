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

pub fn json_to_event(json_event:Json) -> Event {
    let obj_event = json_event.as_object().unwrap();//BTreeMap<String, Json>
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

    Event {
        tuple: Tuple {
            ty_id: ty_id_u.unwrap(),
            data: vec_value,
        },
        time: UTC::now(),
    }
}
// pub fn json_to_event(json_event:Json) -> Event {
pub fn json_to_event_dec(json_event_dec:Json) -> TupleDeclaration {

    let obj_event_dec = json_event_dec.as_object().unwrap();//BTreeMap<String, Json>

    let ty = obj_event_dec.get("ty").unwrap();//Json
    let id = obj_event_dec.get("id").unwrap();//Json
    let name = obj_event_dec.get("name").unwrap();//Json
    let attributes = obj_event_dec.get("attributes").unwrap();//Json

    let ty_str = ty.as_string().unwrap().parse::<usize>().unwrap();
    let id_str = id.as_string().unwrap().parse::<usize>().unwrap();
    let name_str = name.as_string().unwrap();

    let obj_atts = attributes.as_array().unwrap();//Vec<Json>

    let mut vec_att: Vec<AttributeDeclaration> = Vec::new();

    for att in obj_atts.iter(){//Json

        let obj_att = att.as_object().unwrap();

        let att_name = obj_att.get("name").unwrap().as_string().unwrap();
        let att_ty = obj_att.get("ty").unwrap().as_string().unwrap();

        if (att_ty == "Str") {
            vec_att.push(
                AttributeDeclaration {name: att_name.to_owned(), ty: BasicType::Str}
            );
        } else {
            vec_att.push(
                AttributeDeclaration {name: att_name.to_owned(), ty: BasicType::Int}
            );
        }
    }

    TupleDeclaration {
        ty: TupleType::Event,
        id: id_str,
        name: name_str.to_owned(),
        attributes: vec_att,
    }
}



fn to_expression (json_input : Json) -> Expression {

    match X {
        "parameter" => {}
    }

    Expression:: {

    }

}
// fn to_predicate (json_input : Json) -> Predicate {}
// fn to_event_template (json_input : Json) -> EventTemplate {}
// pub fn json_to_rule_def(rule: Json) -> Rule {}




// println!("Rust::defineRule(...)");
// let s = singleton();
// let mut engine = s.inner.lock().unwrap();

// TODO: create the conversion function
// let rule_struct = json_to_rule(rule);
// engine.define(rule_struct);

// engine.define(Rule {
//     predicates: rule_predicate,
//     filters: vec![],
//     event_template: r_e_template,
//     consuming: vec![],
// });
