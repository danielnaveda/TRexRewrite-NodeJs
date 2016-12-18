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
use tesla::expressions::{BasicType, BinaryOperator, UnaryOperator, Expression, Value};
use tesla::predicates::{ConstrainedTuple, EventSelection, ParameterDeclaration, Predicate,PredicateType, Timing, TimingBound};
use trex::TRex;
use trex::stack::StackProvider;

use std::sync::{Mutex, Once, ONCE_INIT};
use std::{mem};

use conn_queues::{insert_queue, pop_queue, print_queue_status,remove_queue,init_queue};

pub trait JsonConversion {
    fn from_json(json_i : Json) -> Self;
}

impl JsonConversion for Event {
    fn from_json(json_i : Json) -> Self {
        let obj_event = json_i.as_object().unwrap();//BTreeMap<String, Json>
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
}

impl JsonConversion for BasicType {
    fn from_json(json_i : Json) -> Self {
        match json_i.as_object().unwrap().get("BasicType").unwrap().as_string().unwrap() {
            "Int" => {BasicType::Int},
            "Float" => {BasicType::Float},
            "Bool" => {BasicType::Bool},
            "Str" => {BasicType::Str},
            _ => {BasicType::Str}
        }
    }
}

impl JsonConversion for Value {
    fn from_json(json_i : Json) -> Self {
        match json_i.as_object().unwrap().get("type").unwrap().as_string().unwrap() {
            "Int" => {
                Value::Int(
                    json_i.as_object().unwrap().get("value").unwrap().as_string().unwrap().parse::<i32>().unwrap()
                )
            },
            "Float" => {
                Value::Float(
                    json_i.as_object().unwrap().get("value").unwrap().as_string().unwrap().parse::<f32>().unwrap()
                )
            },
            "Bool" => {
                Value::Bool(
                    json_i.as_object().unwrap().get("value").unwrap().as_string().unwrap().parse::<bool>().unwrap()
                )
            },
            "Str" => {
                Value::Str(
                    json_i.as_object().unwrap().get("value").unwrap().as_string().unwrap().parse::<String>().unwrap()
                )
            },
            _ => {
                Value::Str(
                    json_i.as_object().unwrap().get("value").unwrap().as_string().unwrap().parse::<String>().unwrap()
                )
            }
        }
    }
}

impl JsonConversion for UnaryOperator {
    fn from_json(json_i : Json) -> Self {
        match json_i.as_object().unwrap().get("UnaryOperator").unwrap().as_string().unwrap() {
            "Minus" => {UnaryOperator::Minus},
            "Not" => {UnaryOperator::Not},
            _ => {UnaryOperator::Minus}
        }
    }
}

impl JsonConversion for BinaryOperator {
    fn from_json(json_i : Json) -> Self {
        match json_i.as_object().unwrap().get("BinaryOperator").unwrap().as_string().unwrap() {
            "Plus" => {BinaryOperator::Plus},
            "Minus" => {BinaryOperator::Minus},
            "Times" => {BinaryOperator::Times},
            "Division" => {BinaryOperator::Division},
            "Equal" => {BinaryOperator::Equal},
            "NotEqual" => {BinaryOperator::NotEqual},
            "GreaterThan" => {BinaryOperator::GreaterThan},
            "GreaterEqual" => {BinaryOperator::GreaterEqual},
            "LowerThan" => {BinaryOperator::LowerThan},
            "LowerEqual" => {BinaryOperator::LowerEqual},
            _ => {BinaryOperator::Plus}
        }
    }
}

// impl JsonConversion for Expression {
//     fn from_json(json_i : Json) -> Self {
//
//     }
// }
//
// impl JsonConversion for TupleType {
//     fn from_json(json_i : Json) -> Self {
//
//     }
// }
//
// impl JsonConversion for AttributeDeclaration {
//     fn from_json(json_i : Json) -> Self {
//
//     }
// }

impl JsonConversion for TupleDeclaration {
    fn from_json(json_i : Json) -> Self {
        let obj_event_dec = json_i.as_object().unwrap();//BTreeMap<String, Json>

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
}

// impl JsonConversion for EventTemplate {
//     fn from_json(json_i : Json) -> Self {
//
//     }
// }
//
// impl JsonConversion for Rule {
//     fn from_json(json_i : Json) -> Self {
//
//     }
// }
//
// impl JsonConversion for Tuple {
//     fn from_json(json_i : Json) -> Self {
//
//     }
// }
//
// impl JsonConversion for EventSelection {
//     fn from_json(json_i : Json) -> Self {
//
//     }
// }
//
// impl JsonConversion for Aggregator {
//     fn from_json(json_i : Json) -> Self {
//
//     }
// }
//
// impl JsonConversion for ParameterDeclaration {
//     fn from_json(json_i : Json) -> Self {
//
//     }
// }
//
// impl JsonConversion for TimingBound {
//     fn from_json(json_i : Json) -> Self {
//
//     }
// }
//
// impl JsonConversion for Timing {
//     fn from_json(json_i : Json) -> Self {
//
//     }
// }
//
// impl JsonConversion for Order {
//     fn from_json(json_i : Json) -> Self {
//
//     }
// }
//
// impl JsonConversion for Ordering {
//     fn from_json(json_i : Json) -> Self {
//
//     }
// }
//
// impl JsonConversion for PredicateType {
//     fn from_json(json_i : Json) -> Self {
//
//     }
// }
//
// impl JsonConversion for ConstrainedTuple {
//     fn from_json(json_i : Json) -> Self {
//
//     }
// }
//
// impl JsonConversion for Predicate {
//     fn from_json(json_i : Json) -> Self {
//
//     }
// }














// From here this should be remove. Keeping it only for reference



/*
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


// {
//     "type" : <type>,
//      ... depends on type
// }
fn to_expression (json_input : Json) -> Expression {
    let expression_o = json_input.as_object().unwrap();//BTreeMap

    match expression_o.get("type").unwrap().as_string().unwrap() {
        "Immediate" => {
            // TODO: create to_value()
            // let value_ = expression_o.get("value").unwrap().as_object().unwrap();
            // Expression::Immediate {value: Value}
            Expression::Aggregate {}
        },
        "Reference" => {
            let att_value = expression_o.get("attribute").unwrap().as_string().unwrap().parse::<usize>().unwrap();
            Expression::Reference {attribute: att_value}
        },
        "Aggregate" => {
            Expression::Aggregate {}
        },
        "Parameter" => {
            let predicate_value = expression_o.get("predicate").unwrap().as_string().unwrap().parse::<usize>().unwrap();
            let parameter_value = expression_o.get("parameter").unwrap().as_string().unwrap().parse::<usize>().unwrap();
            Expression::Parameter {predicate: predicate_value, parameter: parameter_value}
        },
        "Cast" => {
            // TODO: change this
            // Expression::Cast {ty: BasicType, expression: Box<Expression>}
            Expression::Aggregate {}
        },
        "UnaryOperation" => {
            // TODO: change this
            // Expression::UnaryOperation {operator: UnaryOperator, expression: Box<Expression>}
            Expression::Aggregate {}
        },
        "BinaryOperation" => {
            // TODO: change this
            // Expression::BinaryOperation {operator: BinaryOperator, left: Box<Expression>, right: Box<Expression>}
            Expression::Aggregate {}
        },
        _ => {
            // TODO: change this
            Expression::Aggregate {}
        },
    }
}
*/
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
