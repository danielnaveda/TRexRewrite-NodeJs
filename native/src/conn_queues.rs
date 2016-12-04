use std::collections::HashMap;
use std::io;
use std::sync::{Arc, Mutex, Once, ONCE_INIT};
use std::{mem, thread};
use tesla::{AttributeDeclaration, Engine, Event, EventTemplate, Rule, Tuple, TupleDeclaration,
            TupleType};

#[derive(Clone)]
struct SingletonQueues {
    inner: Arc<Mutex<HashMap<String, Vec<i32>>>>
}

fn singletonqueues() -> SingletonQueues {
    static mut SINGLETON: *const SingletonQueues = 0 as *const SingletonQueues;
    static ONCE: Once = ONCE_INIT;

    unsafe {
        ONCE.call_once(|| {
            let singletonqueues = SingletonQueues {
                inner: Arc::new(Mutex::new(HashMap::new()))
            };
            SINGLETON = mem::transmute(Box::new(singletonqueues));
        });
        (*SINGLETON).clone()
    }
}

pub fn insert_queue(connid: String, value: i32){
    let s = singletonqueues();
    let mut conn_queues = s.inner.lock().unwrap();
    let queue = conn_queues.entry(String::from(connid)).or_insert(vec![]);
    (*queue).insert(0,value);
}
// pub fn pop_queue(connid: String){
pub fn pop_queue(connid: String) -> Option<i32> {
    let s = singletonqueues();
    let mut conn_queues = s.inner.lock().unwrap();
    let queue = conn_queues.entry(String::from(connid)).or_insert(vec![]);
    // println!("{:?}",(*queue).pop());
    (*queue).pop()
}

pub fn print_queue_status(){
    let s = singletonqueues();
    let mut conn_queues = s.inner.lock().unwrap();
    println!("Queue Status:");
    println!("conn_queues.len(): {}", conn_queues.len());
}
