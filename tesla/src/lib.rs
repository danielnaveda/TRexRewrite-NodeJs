extern crate chrono;
extern crate ordered_float;

pub mod expressions;
pub mod predicates;

use chrono::{DateTime, UTC};
use expressions::{BasicType, Expression, Value};
use predicates::Predicate;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub enum TupleType {
    Static,
    Event,
}

#[derive(Clone, Debug)]
pub struct AttributeDeclaration {
    pub name: String,
    pub ty: BasicType,
}

#[derive(Clone, Debug)]
pub struct TupleDeclaration {
    pub ty: TupleType,
    pub id: usize,
    pub name: String,
    pub attributes: Vec<AttributeDeclaration>,
}

#[derive(Clone, Debug)]
pub struct EventTemplate {
    pub ty_id: usize,
    pub attributes: Vec<Expression>,
}

#[derive(Clone, Debug)]
pub struct Rule {
    pub predicates: Vec<Predicate>,
    pub filters: Vec<Expression>,
    pub event_template: EventTemplate,
    pub consuming: Vec<usize>,
}

#[derive(Clone, Debug)]
pub struct Tuple {
    pub ty_id: usize,
    pub data: Vec<Value>,
}

#[derive(Clone, Debug)]
pub struct Event {
    pub tuple: Tuple,
    pub time: DateTime<UTC>,
}

#[derive(Clone, Debug)]
pub enum SubscrFilter {
    Any,
    Topic { ty: usize },
    Content { ty: usize, filters: Vec<Expression> },
}

pub trait Listener {
    fn receive(&mut self, event: &Arc<Event>);
}

pub trait Engine {
    fn tuple_id(&mut self, name: &str) -> Option<usize>;
    fn tupleattr_id(&mut self, tuple_name: &str, attr_name: &str) -> Option<usize>;
    fn declare(&mut self, tuple: TupleDeclaration);
    fn define(&mut self, rule: Rule);
    fn publish(&mut self, event: &Arc<Event>);
    fn subscribe(&mut self, condition: SubscrFilter, listener: Box<Listener>) -> usize;
    fn unsubscribe(&mut self, listener_id: usize);
}
