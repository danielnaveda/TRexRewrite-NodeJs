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
    pub filters: Vec<Arc<Expression>>,
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

pub trait ClonableIterator<'a>: Iterator {
    fn clone_iter(&self) -> Box<ClonableIterator<'a, Item = Self::Item> + 'a>;
}

impl<'a, T> ClonableIterator<'a> for T
    where T: Iterator + Clone + 'a
{
    fn clone_iter(&self) -> Box<ClonableIterator<'a, Item = Self::Item> + 'a> {
        Box::new(self.clone())
    }
}

impl<'a, T: 'a> Clone for Box<ClonableIterator<'a, Item = T> + 'a> {
    fn clone(&self) -> Self { (**self).clone_iter() }
}

pub type EventsIterator<'a> = Box<ClonableIterator<'a, Item = &'a Arc<Event>> + 'a>;

pub trait Listener {
    fn receive(&mut self, event: &Arc<Event>);
    fn receive_all(&mut self, events: EventsIterator) {
        for event in events {
            self.receive(event);
        }
    }
}

pub trait Engine {
    fn declare(&mut self, tuple: TupleDeclaration);
    fn define(&mut self, rule: Rule);
    fn publish(&mut self, event: &Arc<Event>);
    fn publish_all(&mut self, events: EventsIterator) {
        for event in events {
            self.publish(event);
        }
    }
    fn subscribe(&mut self, listener: Box<Listener>) -> usize;
    fn unsubscribe(&mut self, listener_id: &usize) -> Option<Box<Listener>>;
}
