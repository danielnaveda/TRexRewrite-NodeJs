use std::f64;
use std::ops::Add;
use std::ops::Deref;
use tesla::{AttributeDeclaration, Event};
use tesla::expressions::{BasicType, Value};
use tesla::predicates::Aggregator;

fn compute_average<'a, T, U>(iterator: T,
                             attributes: &[AttributeDeclaration],
                             attr: usize)
                             -> Option<Value>
    where T: Iterator<Item = &'a U>,
          U: Deref<Target = Event> + 'a
{
    match attributes[attr].ty {
        BasicType::Int => {
            let mapped = iterator.map(|evt| evt.tuple.data[attr].unwrap_int());
            let (count, sum) = mapped.fold((0i64, 0), |acc, x| (acc.0 + 1, acc.1 + x));
            if count > 0 { Some(Value::from(sum as f64 / count as f64)) } else { None }
        }
        BasicType::Float => {
            let mapped = iterator.map(|evt| evt.tuple.data[attr].unwrap_float());
            let (count, sum) = mapped.fold((0i64, 0.0), |acc, x| (acc.0 + 1, acc.1 + x));
            if count > 0 { Some(Value::from(sum / count as f64)) } else { None }
        }
        _ => panic!("Tring to compute aggregate on wrong Value type"),
    }
}

fn compute_sum<'a, T, U>(iterator: T,
                         attributes: &[AttributeDeclaration],
                         attr: usize)
                         -> Option<Value>
    where T: Iterator<Item = &'a U>,
          U: Deref<Target = Event> + 'a
{
    match attributes[attr].ty {
        BasicType::Int => {
            let mapped = iterator.map(|evt| evt.tuple.data[attr].unwrap_int());
            Some(Value::from(mapped.fold(0, Add::add)))
        }
        BasicType::Float => {
            let mapped = iterator.map(|evt| evt.tuple.data[attr].unwrap_float());
            Some(Value::from(mapped.fold(0.0, Add::add)))
        }
        _ => panic!("Tring to compute aggregate on wrong Value type"),
    }
}

fn compute_min<'a, T, U>(iterator: T,
                         attributes: &[AttributeDeclaration],
                         attr: usize)
                         -> Option<Value>
    where T: Iterator<Item = &'a U>,
          U: Deref<Target = Event> + 'a
{
    match attributes[attr].ty {
        BasicType::Int => {
            let mapped = iterator.map(|evt| evt.tuple.data[attr].unwrap_int());
            mapped.min().map(Value::from)
        }
        BasicType::Float => {
            let mapped = iterator.map(|evt| evt.tuple.data[attr].unwrap_float());
            let min = mapped.fold(f64::NAN, f64::min);
            if !min.is_nan() { Some(Value::from(min)) } else { None }
        }
        _ => panic!("Tring to compute aggregate on wrong Value type"),
    }
}

fn compute_max<'a, T, U>(iterator: T,
                         attributes: &[AttributeDeclaration],
                         attr: usize)
                         -> Option<Value>
    where T: Iterator<Item = &'a U>,
          U: Deref<Target = Event> + 'a
{
    match attributes[attr].ty {
        BasicType::Int => {
            let mapped = iterator.map(|evt| evt.tuple.data[attr].unwrap_int());
            mapped.max().map(Value::from)
        }
        BasicType::Float => {
            let mapped = iterator.map(|evt| evt.tuple.data[attr].unwrap_float());
            let max = mapped.fold(f64::NAN, f64::max);
            if !max.is_nan() { Some(Value::from(max)) } else { None }
        }
        _ => panic!("Tring to compute aggregate on wrong Value type"),
    }
}

pub fn compute_aggregate<'a, T, U>(aggregator: &Aggregator,
                                   iterator: T,
                                   attributes: &[AttributeDeclaration])
                                   -> Option<Value>
    where T: Iterator<Item = &'a U>,
          U: Deref<Target = Event> + 'a
{
    match *aggregator {
        Aggregator::Avg(attr) => compute_average(iterator, attributes, attr),
        Aggregator::Sum(attr) => compute_sum(iterator, attributes, attr),
        Aggregator::Min(attr) => compute_min(iterator, attributes, attr),
        Aggregator::Max(attr) => compute_max(iterator, attributes, attr),
        Aggregator::Count => Some(Value::from(iterator.count() as i64)),
    }
}
