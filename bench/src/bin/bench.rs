#![feature(step_by,inclusive_range_syntax)]

extern crate rand;
extern crate chrono;
extern crate num_cpus;
extern crate tesla;
extern crate trex;

use chrono::{Duration, UTC};
use rand::Rng;
use std::sync::Arc;
use std::sync::mpsc::sync_channel;
use std::thread;
use tesla::{AttributeDeclaration, Engine, Event, EventTemplate, Listener, Rule, SubscrFilter,
            Tuple, TupleDeclaration, TupleType};
use tesla::expressions::*;
use tesla::predicates::*;
use trex::*;
use trex::stack::StackProvider;

struct Config {
    num_rules: usize,
    num_def: usize,
    num_pred: usize,
    num_events: usize,
    each_prob: f32,
    first_prob: f32,
    min_win: Duration,
    max_win: Duration,
    consuming: bool,
    queue_len: usize,
    evts_per_sec: usize,
}

fn generate_length_declarations<R: Rng>(rng: &mut R, cfg: &Config) -> Vec<TupleDeclaration> {
    let mut decls = Vec::new();
    for i in 0..cfg.num_def {
        let id = i + 1;
        for j in 0..cfg.num_pred {
            let attr = AttributeDeclaration {
                name: rng.gen_ascii_chars().take(5).collect(),
                ty: BasicType::Int,
            };
            decls.push(TupleDeclaration {
                ty: TupleType::Event,
                id: id * 1000 + j,
                name: rng.gen_ascii_chars().take(5).collect(),
                attributes: vec![attr],
            });
        }
        decls.push(TupleDeclaration {
            ty: TupleType::Event,
            id: id,
            name: rng.gen_ascii_chars().take(5).collect(),
            attributes: Vec::new(),
        });
    }
    decls
}

fn generate_length_rules<R: Rng>(rng: &mut R, cfg: &Config) -> Vec<Rule> {
    let mut rules = Vec::new();
    for i in 0..cfg.num_rules {
        let id = i % cfg.num_def + 1;
        let constraint = Expression::BinaryOperation {
            operator: BinaryOperator::Equal,
            left: Box::new(Expression::Reference { attribute: 0 }),
            right: Box::new(Expression::Immediate { value: 1.into() }),
        };
        let root_pred = Predicate {
            ty: PredicateType::Trigger { parameters: Vec::new() },
            tuple: ConstrainedTuple {
                ty_id: id * 1000,
                constraints: vec![constraint.clone()],
                alias: rng.gen_ascii_chars().take(5).collect(),
            },
        };
        let mut predicates = vec![root_pred];
        for j in 1..cfg.num_pred {
            let rand = rng.gen_range(0.0, 1.0);
            let selection = if 0.0 <= rand && rand < cfg.each_prob {
                EventSelection::Each
            } else if cfg.each_prob <= rand &&
                                      rand < cfg.each_prob + cfg.first_prob {
                EventSelection::First
            } else {
                EventSelection::Last
            };
            predicates.push(Predicate {
                ty: PredicateType::Event {
                    selection: selection,
                    parameters: Vec::new(),
                    timing: Timing {
                        upper: j - 1,
                        bound: TimingBound::Within {
                            window: Duration::milliseconds(
                                rng.gen_range(
                                    cfg.min_win.num_milliseconds(), cfg.max_win.num_milliseconds()
                                )
                            ),
                        },
                    },
                },
                tuple: ConstrainedTuple {
                    ty_id: id * 1000 + j,
                    constraints: vec![constraint.clone()],
                    alias: rng.gen_ascii_chars().take(5).collect(),
                },
            });
        }
        let event_template = EventTemplate {
            ty_id: id,
            attributes: Vec::new(),
        };
        let consuming = if cfg.consuming { vec![1] } else { Vec::new() };
        let rule = Rule {
            predicates: predicates,
            filters: Vec::new(),
            event_template: event_template,
            consuming: consuming,
        };
        rules.push(rule);
    }
    rules
}

fn generate_length_events<R: Rng>(rng: &mut R, cfg: &Config) -> Vec<Event> {
    let mut events = Vec::new();
    for _ in 0..cfg.num_events {
        let def = rng.gen_range(0, cfg.num_def) + 1;
        let state = rng.gen_range(0, cfg.num_pred);
        events.push(Event {
            tuple: Tuple {
                ty_id: def * 1000 + state,
                data: vec![Value::Int(1)],
            },
            time: UTC::now(),
        });
    }
    events
}

#[derive(Clone, Debug)]
struct DebugListener;
impl Listener for DebugListener {
    fn receive(&mut self, event: &Arc<Event>) {
        println!("{:?}", event);
    }
}

#[derive(Clone, Debug)]
struct CountListener {
    duration: usize,
    count: usize,
}
impl Drop for CountListener {
    fn drop(&mut self) {
        println!("Count: {:10} - Throughput: {:7}",
                 self.count,
                 self.count / self.duration);
    }
}
impl Listener for CountListener {
    fn receive(&mut self, event: &Arc<Event>) { self.count += 1; }
}

fn execute_bench_length(cfg: &Config) {
    let mut rng = rand::thread_rng();
    let decls = generate_length_declarations(&mut rng, &cfg);
    let rules = generate_length_rules(&mut rng, &cfg);
    let evts = generate_length_events(&mut rng, &cfg);

    let providers: Vec<Box<NodeProvider>> = vec![Box::new(StackProvider)];
    let mut engine = TRex::new(num_cpus::get(), providers);
    for decl in decls {
        engine.declare(decl);
    }
    for rule in rules {
        engine.define(rule);
    }
    // engine.subscribe(Box::new(DebugListener));
    engine.subscribe(SubscrFilter::Any,
                     Box::new(CountListener {
                         count: 0,
                         duration: cfg.num_events / cfg.evts_per_sec,
                     }));

    let start = UTC::now();

    let (tx, rx) = sync_channel(cfg.queue_len);
    let evts_per_sec = cfg.evts_per_sec as u32;
    let thr = thread::spawn(move || {
        let mut dropped = 0;
        for mut evt in evts {
            evt.time = UTC::now();
            if tx.try_send(evt).is_err() {
                dropped += 1;
            }
            thread::sleep(std::time::Duration::new(0, 1000_000_000 / evts_per_sec));
        }
        dropped
    });
    while let Ok(evt) = rx.recv() {
        engine.publish(&Arc::new(evt));
    }

    println!("Dropped: {:2.2}% - Time: {:5}ms",
             thr.join().unwrap() as f32 / cfg.num_events as f32 * 100.0,
             (UTC::now() - start).num_milliseconds());
}

fn main() {
    let mut cfg = Config {
        num_rules: 650,
        num_def: 65,
        num_pred: 3,
        num_events: 40_000,
        each_prob: 1.0,
        first_prob: 0.0,
        min_win: Duration::seconds(0),
        max_win: Duration::seconds(1),
        consuming: false,
        queue_len: 100,
        evts_per_sec: 10000,
    };

    for queue_bound in vec![true, false] {
        println!("");
        for freq in vec![600usize, 800, 1000, 1500, 2500, 4000] {
            cfg.num_events = freq * 60;
            cfg.queue_len = if queue_bound { freq / 10 } else { cfg.num_events + 1 };
            cfg.evts_per_sec = freq;
            println!("");
            println!("- Frequency: {:5} evt/sec | Queue len: {:5}", freq, cfg.queue_len);
            for avg_win in (2...10).step_by(4) {
                cfg.max_win = Duration::seconds(avg_win + 1 as i64);
                cfg.min_win = Duration::seconds(avg_win - 1 as i64);
                print!(" > Avg Window: {:2}s => ", avg_win);
                execute_bench_length(&cfg);
            }
        }
    }
}
