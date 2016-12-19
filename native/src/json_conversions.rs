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
use tesla::predicates::{ConstrainedTuple, EventSelection, ParameterDeclaration, Predicate,PredicateType, Timing, TimingBound, Order, Ordering, Aggregator};
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
        println!("Event::from_json: {:?}", json_i);
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
        println!("BasicType::from_json: {:?}", json_i);
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
        println!("Value::from_json: {:?}", json_i);
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
        println!("UnaryOperator::from_json: {:?}", json_i);
        match json_i.as_object().unwrap().get("UnaryOperator").unwrap().as_string().unwrap() {
            "Minus" => {UnaryOperator::Minus},
            "Not" => {UnaryOperator::Not},
            _ => {UnaryOperator::Minus}
        }
    }
}

impl JsonConversion for BinaryOperator {
    fn from_json(json_i : Json) -> Self {
        println!("BinaryOperator::from_json: {:?}", json_i);
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

impl JsonConversion for Expression {
    fn from_json(json_i : Json) -> Self {
        println!("Expression::from_json: {:?}", json_i);
        match json_i.as_object().unwrap().get("type").unwrap().as_string().unwrap() {
            "Immediate" => {
                Expression::Immediate {
                    value: Value::from_json(json_i.as_object().unwrap().get("value").unwrap().clone())
                }
            },
            "Reference" => {
                Expression::Reference {
                    attribute: json_i.as_object().unwrap().get("value").unwrap().as_string().unwrap().parse::<usize>().unwrap()
                }
            },
            "Aggregate" => {
                Expression::Aggregate
            },
            "Parameter" => {
                Expression::Parameter {
                    predicate: json_i.as_object().unwrap().get("predicate").unwrap().as_string().unwrap().parse::<usize>().unwrap(),
                    parameter: json_i.as_object().unwrap().get("parameter").unwrap().as_string().unwrap().parse::<usize>().unwrap()
                }
            },
            // "Cast" => {
            //     Value::Str(
            //         json_i.as_object().unwrap().get("value").unwrap().as_string().unwrap().parse::<String>().unwrap()
            //     )
            // },
            // "UnaryOperation" => {
            //     Value::Str(
            //         json_i.as_object().unwrap().get("value").unwrap().as_string().unwrap().parse::<String>().unwrap()
            //     )
            // },
            // "BinaryOperation" => {
            //     Value::Str(
            //         json_i.as_object().unwrap().get("value").unwrap().as_string().unwrap().parse::<String>().unwrap()
            //     )
            // },
            _ => {
                Expression::Aggregate
            }
        }
    }
}

impl JsonConversion for TupleType {
    fn from_json(json_i : Json) -> Self {
        println!("TupleType::from_json: {:?}", json_i);
        match json_i.as_object().unwrap().get("TupleType").unwrap().as_string().unwrap() {
            "Static" => {TupleType::Static},
            "Event" => {TupleType::Event},
            _ => {TupleType::Static}
        }
    }
}

impl JsonConversion for AttributeDeclaration {
    fn from_json(json_i : Json) -> Self {
        println!("AttributeDeclaration::from_json: {:?}", json_i);
        AttributeDeclaration {
            name: String::from(json_i.as_object().unwrap().get("name").unwrap().as_string().unwrap()),
            ty: BasicType::from_json(json_i.as_object().unwrap().get("ty").unwrap().clone()),
        }
    }
}

impl JsonConversion for TupleDeclaration {
    fn from_json(json_i : Json) -> Self {
        println!("TupleDeclaration::from_json: {:?}", json_i);
        let mut attributes: Vec<AttributeDeclaration> = Vec::new();

        for attribute in json_i.as_object().unwrap().get("attributes").unwrap().as_array().unwrap().iter(){
            attributes.push(AttributeDeclaration::from_json(attribute.clone()));
        }

        TupleDeclaration {
            ty: TupleType::from_json(json_i.as_object().unwrap().get("ty").unwrap().clone()),
            id: json_i.as_object().unwrap().get("id").unwrap().as_string().unwrap().parse::<usize>().unwrap(),
            name: String::from(json_i.as_object().unwrap().get("name").unwrap().as_string().unwrap()),
            attributes: attributes,
        }
    }
}

impl JsonConversion for EventTemplate {
    fn from_json(json_i : Json) -> Self {
        println!("EventTemplate::from_json: {:?}", json_i);

        let mut expressions : Vec<Expression> = Vec::new();

        for expression in json_i.as_object().unwrap().get("attributes").unwrap().as_array().unwrap().iter() {
            expressions.push(Expression::from_json(expression.clone()));
        }

        EventTemplate {
            ty_id : json_i.as_object().unwrap().get("ty_id").unwrap().as_string().unwrap().parse::<usize>().unwrap(),
            attributes: expressions
        }
    }
}

impl JsonConversion for Rule {
    fn from_json(json_i : Json) -> Self {
        println!("Rule::from_json: {:?}", json_i);

        let mut predicates : Vec<Predicate> = Vec::new();
        let mut filters : Vec<Arc<Expression>> = Vec::new();
        let mut consumings : Vec<usize> = Vec::new();

        for predicate in json_i.as_object().unwrap().get("predicates").unwrap().as_array().unwrap().iter() {
            predicates.push(Predicate::from_json(predicate.clone()));
        }

        for filter in json_i.as_object().unwrap().get("filters").unwrap().as_array().unwrap().iter() {
            filters.push(Arc::new(Expression::from_json(filter.clone())));
        }

        for consuming in json_i.as_object().unwrap().get("consuming").unwrap().as_array().unwrap().iter() {
            consumings.push(consuming.as_string().unwrap().parse::<usize>().unwrap().clone());
        }


        Rule {
            predicates: predicates,
            filters: filters,
            event_template: EventTemplate::from_json(json_i.as_object().unwrap().get("event_template").unwrap().clone()),
            consuming: consumings,
        }
    }
}

impl JsonConversion for Tuple {
    fn from_json(json_i : Json) -> Self {
        println!("Tuple::from_json: {:?}", json_i);

        let mut values : Vec<Value> = Vec::new();

        for value in json_i.as_object().unwrap().get("data").unwrap().as_array().unwrap().iter() {
            values.push(Value::from_json(value.clone()));
        }

        Tuple {
            ty_id: json_i.as_object().unwrap().get("ty_id").unwrap().as_string().unwrap().parse::<usize>().unwrap().clone(),
            data: values,
        }
    }
}

impl JsonConversion for EventSelection {
    fn from_json(json_i : Json) -> Self {
        println!("EventSelection::from_json: {:?}", json_i);
        match json_i.as_object().unwrap().get("EventSelection").unwrap().as_string().unwrap() {
            "Each" => {EventSelection::Each},
            "First" => {EventSelection::First},
            "Last" => {EventSelection::Last},
            _ => {EventSelection::Each}
        }
    }
}

impl JsonConversion for Aggregator {
    fn from_json(json_i : Json) -> Self {
        println!("Aggregator::from_json: {:?}", json_i);
        match json_i.as_object().unwrap().get("Aggregator").unwrap().as_string().unwrap() {
            "Avg" => {
                Aggregator::Avg(
                    json_i.as_object().unwrap().get("value").unwrap().as_string().unwrap().parse::<usize>().unwrap()
                )
            },
            "Sum" => {
                Aggregator::Sum(
                    json_i.as_object().unwrap().get("value").unwrap().as_string().unwrap().parse::<usize>().unwrap()
                )
            },
            "Max" => {
                Aggregator::Max(
                    json_i.as_object().unwrap().get("value").unwrap().as_string().unwrap().parse::<usize>().unwrap()
                )
            },
            "Min" => {
                Aggregator::Min(
                    json_i.as_object().unwrap().get("value").unwrap().as_string().unwrap().parse::<usize>().unwrap()
                )
            },
            _ => {
                Aggregator::Avg(
                    json_i.as_object().unwrap().get("value").unwrap().as_string().unwrap().parse::<usize>().unwrap()
                )
            }
        }
    }
}

impl JsonConversion for ParameterDeclaration {
    fn from_json(json_i : Json) -> Self {
        println!("ParameterDeclaration::from_json: {:?}", json_i);
        ParameterDeclaration {
            name: String::from(json_i.as_object().unwrap().get("name").unwrap().as_string().unwrap()),
            expression: Arc::new(Expression::from_json(json_i.as_object().unwrap().get("expression").unwrap().clone())),
        }
    }
}

impl JsonConversion for TimingBound {
    fn from_json(json_i : Json) -> Self {
        println!("TimingBound::from_json: {:?}", json_i);
        match json_i.as_object().unwrap().get("type").unwrap().as_string().unwrap() {
            "Within" => {
                TimingBound::Within { window: Duration::seconds(json_i.as_object().unwrap().get("lower").unwrap().as_string().unwrap().parse::<i64>().unwrap()) }
            },
            "Between" => {
                TimingBound::Between { lower: json_i.as_object().unwrap().get("lower").unwrap().as_string().unwrap().parse::<usize>().unwrap() }
            },
            _ => {
                TimingBound::Within { window: Duration::seconds(json_i.as_object().unwrap().get("lower").unwrap().as_string().unwrap().parse::<i64>().unwrap()) }
            },
        }
    }
}

impl JsonConversion for Timing {
    fn from_json(json_i : Json) -> Self {
        println!("Timing::from_json: {:?}", json_i);
        Timing {
            upper: json_i.as_object().unwrap().get("upper").unwrap().as_string().unwrap().parse::<usize>().unwrap(),
            bound: TimingBound::from_json(json_i.as_object().unwrap().get("bound").unwrap().clone()),
        }
    }
}

impl JsonConversion for Order {
    fn from_json(json_i : Json) -> Self {
        println!("Order::from_json: {:?}", json_i);
        match json_i.as_object().unwrap().get("Order").unwrap().as_string().unwrap() {
            "Asc" => {Order::Asc},
            "Desc" => {Order::Desc},
            _ => {Order::Asc}
        }
    }
}

impl JsonConversion for Ordering {
    fn from_json(json_i : Json) -> Self {
        println!("Ordering::from_json: {:?}", json_i);
        Ordering {
            attribute: json_i.as_object().unwrap().get("attribute").unwrap().as_string().unwrap().parse::<usize>().unwrap(),
            direction: Order::from_json(json_i.as_object().unwrap().get("order").unwrap().clone()),
        }
    }
}

impl JsonConversion for PredicateType {
    fn from_json(json_i : Json) -> Self {
        println!("PredicateType::from_json: {:?}", json_i);

        match json_i.as_object().unwrap().get("type").unwrap().as_string().unwrap() {
            "Trigger" => {
                let mut parameters : Vec<ParameterDeclaration> = Vec::new();

                for parameter in json_i.as_object().unwrap().get("parameters").unwrap().as_array().unwrap().iter() {
                    parameters.push(ParameterDeclaration::from_json(parameter.clone()));
                }

                PredicateType::Trigger { parameters: parameters }
            },
            "Event" => {
                let mut parameters : Vec<ParameterDeclaration> = Vec::new();

                for parameter in json_i.as_object().unwrap().get("parameters").unwrap().as_array().unwrap().iter() {
                    parameters.push(ParameterDeclaration::from_json(parameter.clone()));
                }

                PredicateType::Event {
                    selection: EventSelection::Each, //TODO: replace this
                    parameters: parameters,
                    timing: Timing { upper: (1 as usize), bound: TimingBound::Between {lower: (1 as usize)}},//TODO: replace this
                }
            },
            _ => {
                let mut parameters : Vec<ParameterDeclaration> = Vec::new();

                for parameter in json_i.as_object().unwrap().get("parameters").unwrap().as_array().unwrap().iter() {
                    parameters.push(ParameterDeclaration::from_json(parameter.clone()));
                }

                PredicateType::Trigger { parameters: parameters }
            },
        }
    }
}

impl JsonConversion for ConstrainedTuple {
    fn from_json(json_i : Json) -> Self {
        println!("ConstrainedTuple::from_json: {:?}", json_i);

        let mut expressions : Vec<Arc<Expression>> = Vec::new();

        for expression in json_i.as_object().unwrap().get("constraints").unwrap().as_array().unwrap().iter() {
            expressions.push(Arc::new(Expression::from_json(expression.clone())));
        }

        ConstrainedTuple {
            ty_id: json_i.as_object().unwrap().get("ty_id").unwrap().as_string().unwrap().parse::<usize>().unwrap().clone(),
            constraints: expressions,
            alias: String::from(json_i.as_object().unwrap().get("alias").unwrap().as_string().unwrap()),
        }
    }
}

impl JsonConversion for Predicate {
    fn from_json(json_i : Json) -> Self {
        println!("Predicate::from_json: {:?}", json_i);
        Predicate {
            ty: PredicateType::from_json(json_i.as_object().unwrap().get("ty").unwrap().clone()),
            tuple: ConstrainedTuple::from_json(json_i.as_object().unwrap().get("tuple").unwrap().clone()),
        }
    }
}














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
