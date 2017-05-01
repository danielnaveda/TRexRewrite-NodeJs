extern crate rustc_serialize;
extern crate chrono;
extern crate num_cpus;
extern crate tesla;
extern crate trex;

use rustc_serialize::json::Json;
use chrono::{Duration, UTC};
use std::sync::Arc;
use tesla::{AttributeDeclaration, Event, EventTemplate, Rule, Tuple, TupleDeclaration,TupleType};
use tesla::expressions::{BasicType, BinaryOperator, UnaryOperator, Expression, Value};
use tesla::predicates::{ConstrainedTuple, EventSelection, ParameterDeclaration, Predicate,PredicateType, Timing, TimingBound, Order, Ordering, Aggregator};
use operations::{get_tuple_id, get_tupleattr_id};

pub trait JsonConversion {
    fn from_json(json_i : Json) -> Self;
}

impl JsonConversion for Event {
    fn from_json(json_i : Json) -> Self {
        println!("Event::from_json: {:?}\n", json_i);
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
        println!("BasicType::from_json: {:?}\n", json_i);

        match json_i.as_string().unwrap() {
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
        println!("Value::from_json: {:?}\n", json_i);
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
        println!("UnaryOperator::from_json: {:?}\n", json_i);
        match json_i.as_object().unwrap().get("UnaryOperator").unwrap().as_string().unwrap() {
            "Minus" => {UnaryOperator::Minus},
            "Not" => {UnaryOperator::Not},
            _ => {UnaryOperator::Minus}
        }
    }
}

impl JsonConversion for BinaryOperator {
    fn from_json(json_i : Json) -> Self {
        println!("BinaryOperator::from_json: {:?}\n", json_i);
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
        println!("Expression::from_json: {:?}\n", json_i);

        if json_i.as_object().unwrap().contains_key("Immediate") {
            Expression::Immediate {
                value: Value::from_json(json_i.as_object().unwrap().get("value").unwrap().clone())
            }
        } else if json_i.as_object().unwrap().contains_key("Reference") {
            let attribute_t_v = json_i.as_object().unwrap().get("Reference").unwrap()
                                          .as_object().unwrap().get("attribute").unwrap()
                                          .as_string().unwrap();

            let tuple_name = attribute_t_v.split("::").nth(0).unwrap();
            let attr_name = attribute_t_v.split("::").nth(1).unwrap();

            Expression::Reference {
                attribute: get_tupleattr_id(tuple_name, attr_name).unwrap()
            }
        } else if json_i.as_object().unwrap().contains_key("Parameter") {
            Expression::Parameter {
                predicate: (json_i.as_object().unwrap().get("Parameter").unwrap()
                                 .as_object().unwrap().get("predicate").unwrap()
                                 .as_u64().unwrap() as usize),
                parameter: (json_i.as_object().unwrap().get("Parameter").unwrap()
                                  .as_object().unwrap().get("parameter").unwrap()
                                  .as_u64().unwrap() as usize)
            }
        } else if json_i.as_object().unwrap().contains_key("BinaryOperation") {
            let operator_str = json_i.as_object().unwrap().get("BinaryOperation").unwrap()
                            .as_object().unwrap().get("operator").unwrap()
                            .as_string().unwrap();

            let mut operator = BinaryOperator::GreaterThan;
            if operator_str == "Plus" {
                operator = BinaryOperator::Plus;
            } else if operator_str == "Minus" {
                operator = BinaryOperator::Minus;
            } else if operator_str == "Times" {
                operator = BinaryOperator::Times;
            } else if operator_str == "Division" {
                operator = BinaryOperator::Division;
            } else if operator_str == "Equal" {
                operator = BinaryOperator::Equal;
            } else if operator_str == "NotEqual" {
                operator = BinaryOperator::NotEqual;
            } else if operator_str == "GreaterThan" {
                operator = BinaryOperator::GreaterThan;
            } else if operator_str == "GreaterEqual" {
                operator = BinaryOperator::GreaterEqual;
            } else if operator_str == "LowerThan" {
                operator = BinaryOperator::LowerThan;
            } else if operator_str == "LowerEqual" {
                operator = BinaryOperator::LowerEqual;
            }

            let mut right = Box::new(Expression::Immediate {value: Value::Int(45),});

            if json_i.as_object().unwrap().get("BinaryOperation").unwrap()
                  .as_object().unwrap().get("right").unwrap()
                  .as_object().unwrap().contains_key("Parameter") {
                      right = Box::new(Expression::Parameter {
                          predicate: json_i.as_object().unwrap().get("BinaryOperation").unwrap()
                                           .as_object().unwrap().get("right").unwrap()
                                           .as_object().unwrap().get("Parameter").unwrap()
                                           .as_object().unwrap().get("predicate").unwrap()
                                           .as_u64().unwrap() as usize,
                          parameter: json_i.as_object().unwrap().get("BinaryOperation").unwrap()
                                           .as_object().unwrap().get("right").unwrap()
                                           .as_object().unwrap().get("Parameter").unwrap()
                                           .as_object().unwrap().get("parameter").unwrap()
                                           .as_u64().unwrap() as usize
                      });
                  }


              let attribute_t_v = json_i.as_object().unwrap().get("BinaryOperation").unwrap()
                                            .as_object().unwrap().get("left").unwrap()
                                            .as_object().unwrap().get("Reference").unwrap()
                                            .as_object().unwrap().get("attribute").unwrap()
                                            .as_string().unwrap();

              let tuple_name = attribute_t_v.split("::").nth(0).unwrap();
              let attr_name = attribute_t_v.split("::").nth(1).unwrap();


            Expression::BinaryOperation {
                operator: operator,
                left: Box::new(Expression::Reference {attribute: get_tupleattr_id(tuple_name, attr_name).unwrap(),}),
                right: right,
            }
        } else {
            Expression::Reference {
                attribute: (json_i.as_object().unwrap().get("Reference").unwrap()
                                 .as_object().unwrap().get("attribute").unwrap()
                                 .as_u64().unwrap() as usize)
            }
        }
    }
}

impl JsonConversion for TupleType {
    fn from_json(json_i : Json) -> Self {
        println!("TupleType::from_json: {:?}\n", json_i);

        match json_i.as_string().unwrap() {
            "Static" => {TupleType::Static},
            "Event" => {TupleType::Event},
            _ => {TupleType::Static}
        }
    }
}

impl JsonConversion for AttributeDeclaration {
    fn from_json(json_i : Json) -> Self {
        println!("AttributeDeclaration::from_json: {:?}\n", json_i);
        AttributeDeclaration {
            name: String::from(json_i.as_object().unwrap().get("name").unwrap().as_string().unwrap()),
            ty: BasicType::from_json(json_i.as_object().unwrap().get("ty").unwrap().clone()),
        }
    }
}

impl JsonConversion for TupleDeclaration {
    fn from_json(json_i : Json) -> Self {
        println!("TupleDeclaration::from_json: {:?}\n", json_i);
        let mut attributes: Vec<AttributeDeclaration> = Vec::new();

        for attribute in json_i.as_object().unwrap().get("attributes").unwrap().as_array().unwrap().iter(){
            attributes.push(AttributeDeclaration::from_json(attribute.clone()));
        }

        TupleDeclaration {
            ty: TupleType::from_json(json_i.as_object().unwrap().get("ty").unwrap().clone()),
            id: (json_i.as_object().unwrap().get("id").unwrap().as_u64().unwrap() as usize),
            name: String::from(json_i.as_object().unwrap().get("name").unwrap().as_string().unwrap()),
            attributes: attributes,
        }
    }
}

impl JsonConversion for EventTemplate {
    fn from_json(json_i : Json) -> Self {
        println!("EventTemplate::from_json: {:?}\n", json_i);

        let mut expressions : Vec<Expression> = Vec::new();

        for expression in json_i.as_object().unwrap().get("attributes").unwrap().as_array().unwrap().iter() {
            expressions.push(Expression::from_json(expression.clone()));
        }

        EventTemplate {
            ty_id : get_tuple_id(json_i.as_object().unwrap().get("ty_id").unwrap().as_string().unwrap()).unwrap(),
            attributes: expressions
        }
    }
}

impl JsonConversion for Rule {
    fn from_json(json_i : Json) -> Self {
        println!("Rule::from_json: {:?}\n", json_i);

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
        println!("Tuple::from_json: {:?}\n", json_i);

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
        println!("EventSelection::from_json: {:?}\n", json_i);

        match json_i.as_string().unwrap() {
            "Each" => {EventSelection::Each},
            "First" => {EventSelection::First},
            "Last" => {EventSelection::Last},
            _ => {EventSelection::Each}
        }
    }
}

impl JsonConversion for Aggregator {
    fn from_json(json_i : Json) -> Self {
        println!("Aggregator::from_json: {:?}\n", json_i);
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
        println!("ParameterDeclaration::from_json: {:?}\n", json_i);
        ParameterDeclaration {
            name: String::from(json_i.as_object().unwrap().get("name").unwrap().as_string().unwrap()),
            expression: Arc::new(Expression::from_json(json_i.as_object().unwrap().get("expression").unwrap().clone())),
        }
    }
}

impl JsonConversion for TimingBound {
    fn from_json(json_i : Json) -> Self {
        println!("TimingBound::from_json: {:?}\n", json_i);
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
        println!("Timing::from_json: {:?}\n", json_i);
        Timing {
            upper: json_i.as_object().unwrap().get("upper").unwrap().as_string().unwrap().parse::<usize>().unwrap(),
            bound: TimingBound::from_json(json_i.as_object().unwrap().get("bound").unwrap().clone()),
        }
    }
}

impl JsonConversion for Order {
    fn from_json(json_i : Json) -> Self {
        println!("Order::from_json: {:?}\n", json_i);
        match json_i.as_object().unwrap().get("Order").unwrap().as_string().unwrap() {
            "Asc" => {Order::Asc},
            "Desc" => {Order::Desc},
            _ => {Order::Asc}
        }
    }
}

impl JsonConversion for Ordering {
    fn from_json(json_i : Json) -> Self {
        println!("Ordering::from_json: {:?}\n", json_i);
        Ordering {
            attribute: json_i.as_object().unwrap().get("attribute").unwrap().as_string().unwrap().parse::<usize>().unwrap(),
            direction: Order::from_json(json_i.as_object().unwrap().get("order").unwrap().clone()),
        }
    }
}

impl JsonConversion for PredicateType {
    fn from_json(json_i : Json) -> Self {
        println!("PredicateType::from_json: {:?}\n", json_i);

        if json_i.as_object().unwrap().contains_key("Trigger") {
            let mut parameters : Vec<ParameterDeclaration> = Vec::new();

            for parameter in json_i.as_object().unwrap().get("Trigger").unwrap()
                                   .as_object().unwrap().get("parameters").unwrap()
                                   .as_array().unwrap().iter() {
                parameters.push(ParameterDeclaration::from_json(parameter.clone()));
            }

            PredicateType::Trigger { parameters: parameters }
        } else if json_i.as_object().unwrap().contains_key("Event") {
            let mut parameters : Vec<ParameterDeclaration> = Vec::new();

            for parameter in json_i.as_object().unwrap().get("Event").unwrap()
                                   .as_object().unwrap().get("parameters").unwrap()
                                   .as_array().unwrap().iter() {
                parameters.push(ParameterDeclaration::from_json(parameter.clone()));
            }


            let bound_temp : TimingBound;
            if json_i.as_object().unwrap().get("Event").unwrap()
                  .as_object().unwrap().get("timing").unwrap()
                  .as_object().unwrap().get("bound").unwrap()
                  .as_object().unwrap().contains_key("Between") {
                      bound_temp = TimingBound::Between {
                          lower: (
                              json_i.as_object().unwrap().get("Event").unwrap()
                                    .as_object().unwrap().get("timing").unwrap()
                                    .as_object().unwrap().get("bound").unwrap()
                                    .as_object().unwrap().get("Between").unwrap()
                                    .as_object().unwrap().get("lower").unwrap()
                                    .as_u64().unwrap()
                                    as usize)
                      };
                  } else {
                      bound_temp = TimingBound::Within {
                          window: Duration::minutes(
                              json_i.as_object().unwrap().get("Event").unwrap()
                                    .as_object().unwrap().get("timing").unwrap()
                                    .as_object().unwrap().get("bound").unwrap()
                                    .as_object().unwrap().get("Within").unwrap()
                                    .as_object().unwrap().get("window").unwrap()
                                    .as_u64().unwrap()
                                    as i64)
                      };
                  }

            PredicateType::Event {
                selection: EventSelection::from_json(json_i.as_object().unwrap().get("Event").unwrap()
                                       .as_object().unwrap().get("selection").unwrap().clone()),
                parameters: parameters,
                timing: Timing {
                    upper: (
                        json_i.as_object().unwrap().get("Event").unwrap()
                              .as_object().unwrap().get("timing").unwrap()
                              .as_object().unwrap().get("upper").unwrap()
                              .as_u64().unwrap()
                              as usize),
                    bound: bound_temp
                }
            }
        } else {
            let mut parameters : Vec<ParameterDeclaration> = Vec::new();

            for parameter in json_i.as_object().unwrap().get("parameters").unwrap().as_array().unwrap().iter() {
                parameters.push(ParameterDeclaration::from_json(parameter.clone()));
            }

            PredicateType::Trigger { parameters: parameters }
        }
    }
}

impl JsonConversion for ConstrainedTuple {
    fn from_json(json_i : Json) -> Self {
        println!("ConstrainedTuple::from_json: {:?}\n", json_i);

        let mut expressions : Vec<Arc<Expression>> = Vec::new();

        for expression in json_i.as_object().unwrap().get("constraints").unwrap().as_array().unwrap().iter() {
            expressions.push(Arc::new(Expression::from_json(expression.clone())));
        }

        ConstrainedTuple {
            ty_id: get_tuple_id(json_i.as_object().unwrap().get("ty_id").unwrap().as_string().unwrap()).unwrap(),
            constraints: expressions,
            alias: String::from(json_i.as_object().unwrap().get("alias").unwrap().as_string().unwrap()),
        }
    }
}

impl JsonConversion for Predicate {
    fn from_json(json_i : Json) -> Self {
        println!("Predicate::from_json: {:?}\n", json_i);
        Predicate {
            ty: PredicateType::from_json(json_i.as_object().unwrap().get("ty").unwrap().clone()),
            tuple: ConstrainedTuple::from_json(json_i.as_object().unwrap().get("tuple").unwrap().clone()),
        }
    }
}
