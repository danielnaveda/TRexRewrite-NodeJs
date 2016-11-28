use std::io;
use std::sync::{Arc, Mutex, Once, ONCE_INIT};
use std::{mem, thread};
use tesla::{AttributeDeclaration, Engine, Event, EventTemplate, Rule, Tuple, TupleDeclaration,
            TupleType};

#[derive(Clone)]
struct SingletonVector {
    // inner: Arc<Mutex<Vec<Vec<i32>>>>
    inner: Arc<Mutex<Vec<Vec<Event>>>>
}

fn vectorsingleton() -> SingletonVector {
    static mut SINGLETON: *const SingletonVector = 0 as *const SingletonVector;
    static ONCE: Once = ONCE_INIT;

    unsafe {
        ONCE.call_once(|| {
            let vectorsingleton = SingletonVector {
                inner: Arc::new(Mutex::new(vec![]))
            };
            SINGLETON = mem::transmute(Box::new(vectorsingleton));
        });
        (*SINGLETON).clone()
    }
}

pub fn m_subscribe() {
    let s = vectorsingleton();
    let mut vector_var = s.inner.lock().unwrap();
    vector_var.push(vec![]);
}

// pub fn m_publish(value: i32) {
pub fn m_publish(value: Event) {
    let s = vectorsingleton();
    let mut vector_var = s.inner.lock().unwrap();
    // vector_var.push(vec![]);

    for i in 0..vector_var.len() {
        vector_var[i].push(value.clone());
    }
}

// pub fn m_get_publish(id: usize) -> Option<i32> {
pub fn m_get_publish(id: usize) -> Option<Event> {
    let s = vectorsingleton();
    let mut vector_var = s.inner.lock().unwrap();
    vector_var[id].pop()
}
