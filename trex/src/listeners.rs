use std::sync::Arc;
use tesla::{Event, Listener};

#[derive(Clone, Debug)]
pub struct DebugListener;
impl Listener for DebugListener {
    fn receive(&mut self, event: &Arc<Event>) {
        println!("{:?}", event);
    }
}

#[derive(Clone, Debug)]
pub struct CountListener {
    pub duration: usize,
    pub count: usize,
}
impl Drop for CountListener {
    fn drop(&mut self) {
        println!("Count: {:10} - Throughput: {:7}",
                 self.count,
                 self.count / self.duration);
    }
}
impl Listener for CountListener {
    fn receive(&mut self, _: &Arc<Event>) { self.count += 1; }
}
