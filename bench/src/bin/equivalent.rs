
extern crate chrono;
extern crate clap;
extern crate num_cpus;
extern crate rand;
extern crate regex;
extern crate rusqlite;
extern crate tesla;
extern crate trex;

use chrono::{Duration, UTC};
use clap::{App, Arg};
use rand::{Rng, SeedableRng, StdRng};
use rand::distributions::{Exp, IndependentSample, Normal, Range, Sample};
use regex::Regex;
use std::fmt;
use std::iter::{once, repeat};
use std::sync::Arc;
use std::sync::mpsc::sync_channel;
use std::thread;
use tesla::{AttributeDeclaration, Engine, Event, EventTemplate, Rule, SubscrFilter, Tuple,
            TupleDeclaration, TupleType};
use tesla::expressions::*;
use tesla::predicates::*;
use trex::*;
use trex::sqlite::{CacheOwnership, CacheType, SqliteConfig, SqliteProvider};
use trex::stack::StackProvider;

enum QueryDistribution {
    Normal(Normal),
    Exp(Exp),
    Uniform(Range<i64>),
}

impl fmt::Debug for QueryDistribution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            QueryDistribution::Normal(..) => write!(f, "Normal"),
            QueryDistribution::Exp(..) => write!(f, "Exp"),
            QueryDistribution::Uniform(..) => write!(f, "Uniform"),
        }
    }
}

impl Sample<i64> for QueryDistribution {
    fn sample<R: Rng>(&mut self, rng: &mut R) -> i64 {
        self.ind_sample(rng)
    }
}

impl IndependentSample<i64> for QueryDistribution {
    fn ind_sample<R: Rng>(&self, rng: &mut R) -> i64 {
        match *self {
            QueryDistribution::Normal(ref distr) => distr.ind_sample(rng) as i64,
            QueryDistribution::Exp(ref distr) => distr.ind_sample(rng) as i64,
            QueryDistribution::Uniform(ref distr) => distr.ind_sample(rng),
        }
    }
}

#[derive(Debug)]
struct Config {
    seed: usize,
    threads: usize,
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
    db_name: String,
    table_columns: usize,
    table_rows: usize,
    table_indexed: bool,
    cache_size: usize,
    cache_ownership: CacheOwnership,
    cache_type: CacheType,
    query_distribution: QueryDistribution,
    matching_rows: usize,
    query_dependencies: usize,
    static_prob: f32,
}

fn db_equivalent<R: Rng>(rng: &mut R, cfg: &Config) -> Vec<Event> {
    let n_rows = cfg.table_rows as i64;
    (0..n_rows)
        .map(|i| {
            let data = once(i)
                .chain(repeat(rng.gen_range(-1 * n_rows / 2, n_rows / 2))
                    .take(cfg.table_columns))
                .map(From::from)
                .collect();
            Event {
                tuple: Tuple {
                    ty_id: 0,
                    data: data,
                },
                time: UTC::now(),
            }
        })
        .collect()
}

fn generate_declarations<R: Rng>(rng: &mut R, cfg: &Config) -> Vec<TupleDeclaration> {
    let attributes = (0..cfg.table_columns)
        .map(|i| {
            AttributeDeclaration {
                name: format!("col{}", i),
                ty: BasicType::Int,
            }
        })
        .collect();
    let static_decl = TupleDeclaration {
        ty: TupleType::Event,
        id: 0,
        name: "test".to_owned(),
        attributes: attributes,
    };
    (0..cfg.num_def)
        .flat_map(|i| {
            let id = i + 1;
            let output_decl = TupleDeclaration {
                ty: TupleType::Event,
                id: id,
                name: format!("event{}", id),
                attributes: vec![
                    AttributeDeclaration {
                        name: "attr".to_owned(),
                        ty: BasicType::Int,
                    },
                ],
            };
            let mid_decls = (0..cfg.num_pred).map(move |j| {
                let num_attr = if j == cfg.num_pred - 1 { 3 } else { 2 };
                let attrs = (0..num_attr)
                    .map(|j| {
                        AttributeDeclaration {
                            name: format!("attr{}", j),
                            ty: BasicType::Int,
                        }
                    })
                    .collect();
                TupleDeclaration {
                    ty: TupleType::Event,
                    id: id * 1000 + j,
                    name: format!("event{}", id * 1000 + j),
                    attributes: attrs,
                }
            });
            once(output_decl).chain(mid_decls)
        })
        .chain(once(static_decl))
        .collect()
}

fn generate_rules<R: Rng>(rng: &mut R, cfg: &Config) -> Vec<Rule> {
    (0..cfg.num_rules)
        .map(|i| {
            let id = i % cfg.num_def + 1;
            let constraint = Expression::BinaryOperation {
                operator: BinaryOperator::Equal,
                left: Box::new(Expression::Reference { attribute: 0 }),
                right: Box::new(Expression::Immediate { value: 1.into() }),
            };

            let root_pred = Predicate {
                ty: PredicateType::Trigger {
                    parameters: vec![
                        ParameterDeclaration {
                            name: "param0x1".to_owned(),
                            expression: Expression::Reference { attribute: 1 },
                        },
                    ],
                },
                tuple: ConstrainedTuple {
                    ty_id: id * 1000,
                    constraints: vec![constraint.clone()],
                    alias: format!("alias{}", id * 1000),
                },
            };

            let last_pred = if rng.next_f32() <= cfg.static_prob {
                let static_constr1 = Expression::BinaryOperation {
                    operator: BinaryOperator::GreaterEqual,
                    left: Box::new(Expression::Reference { attribute: 0 }),
                    right: Box::new(Expression::Parameter {
                        predicate: cfg.num_pred - 1,
                        parameter: 0,
                    }),
                };
                let static_constr2 = Expression::BinaryOperation {
                    operator: BinaryOperator::LowerThan,
                    left: Box::new(Expression::Reference { attribute: 0 }),
                    right: Box::new(Expression::Parameter {
                        predicate: cfg.num_pred - 1,
                        parameter: 1,
                    }),
                };
                let other_constr = (1..cfg.query_dependencies).map(|j| {
                    Expression::BinaryOperation {
                        operator: BinaryOperator::LowerThan,
                        left: Box::new(Expression::Parameter {
                            predicate: cfg.num_pred - 1 - j,
                            parameter: 0,
                        }),
                        right: Box::new(Expression::Immediate { value: 1_000_000_000.into() }),
                    }
                });
                let parameters = (0..cfg.table_columns)
                    .map(|j| {
                        ParameterDeclaration {
                            name: format!("param{}x{}", cfg.num_pred, j),
                            expression: Expression::Reference { attribute: j },
                        }
                    })
                    .collect();
                Some(Predicate {
                    ty: PredicateType::Event {
                        selection: EventSelection::Each,
                        parameters: parameters,
                        timing: Timing {
                            upper: cfg.num_pred - 1,
                            bound: TimingBound::Within { window: Duration::days(100) },
                        },
                    },
                    tuple: ConstrainedTuple {
                        ty_id: 0,
                        constraints: once(static_constr1)
                            .chain(once(static_constr2))
                            .chain(other_constr)
                            .collect(),
                        alias: format!("alias{}", 0),
                    },
                })
            } else {
                None
            };

            let mid_preds = (1..cfg.num_pred).map(|j| {
                let rand = rng.next_f32();
                let selection = if rand < cfg.each_prob {
                    EventSelection::Each
                } else if rand < cfg.each_prob + cfg.first_prob {
                    EventSelection::First
                } else {
                    EventSelection::Last
                };
                let millis = rng.gen_range(cfg.min_win.num_milliseconds(),
                                           cfg.max_win.num_milliseconds());
                let timing = Timing {
                    upper: j - 1,
                    bound: TimingBound::Within { window: Duration::milliseconds(millis) },
                };
                let num_param = if j == cfg.num_pred - 1 { 2 } else { 1 };
                let params = (0..num_param)
                    .map(|k| {
                        ParameterDeclaration {
                            name: format!("param{}x{}", j, k),
                            expression: Expression::Reference { attribute: k + 1 },
                        }
                    })
                    .collect();
                Predicate {
                    ty: PredicateType::Event {
                        selection: selection,
                        parameters: params,
                        timing: timing,
                    },
                    tuple: ConstrainedTuple {
                        ty_id: id * 1000 + j,
                        constraints: vec![constraint.clone()],
                        alias: format!("alias{}", id * 1000 + j),
                    },
                }
            });

            let predicates = once(root_pred).chain(mid_preds).chain(last_pred).collect();
            let event_template = EventTemplate {
                ty_id: id,
                attributes: vec![
                    Expression::Parameter {
                        predicate: cfg.num_pred,
                        parameter: 0,
                    },
                ],
            };
            let consuming = if cfg.consuming { vec![1] } else { Vec::new() };
            Rule {
                predicates: predicates,
                filters: Vec::new(),
                event_template: event_template,
                consuming: consuming,
            }
        })
        .collect()
}

fn generate_events<R: Rng>(rng: &mut R, cfg: &Config) -> Vec<Event> {
    (0..cfg.num_events)
        .map(|_| {
            let def = rng.gen_range(0, cfg.num_def) + 1;
            let state = rng.gen_range(0, cfg.num_pred);

            let random = cfg.query_distribution.ind_sample(rng);
            let upper_bound = if state == (cfg.num_pred - 1) {
                let mut pseudo_rng = StdRng::from_seed(&[random as usize]);
                Some(random + pseudo_rng.gen_range(3, cfg.matching_rows as i64 * 2))
            } else {
                None
            };

            Event {
                tuple: Tuple {
                    ty_id: def * 1000 + state,
                    data: once(1).chain(once(random)).chain(upper_bound).map(From::from).collect(),
                },
                time: UTC::now(),
            }
        })
        .collect()
}

fn execute_bench<R: Rng>(rng: &mut R, cfg: &Config) {
    let db_eq = db_equivalent(rng, cfg);
    let decls = generate_declarations(rng, cfg);
    let rules = generate_rules(rng, cfg);
    let evts = generate_events(rng, cfg);

    let mut engine = TRex::new(cfg.threads, vec![Box::new(StackProvider)]);
    for decl in decls {
        engine.declare(decl);
    }
    for rule in rules {
        engine.define(rule);
    }

    for event in db_eq {
        engine.publish(&Arc::new(event));
    }

    use trex::listeners::{CountListener, DebugListener};
    // engine.subscribe(Box::new(DebugListener));
    engine.subscribe(SubscrFilter::Any, Box::new(CountListener {
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
    let matches = App::new("TRex Equivalent Testing")
        .arg(Arg::with_name("seed")
            .long("seed")
            .value_name("INT")
            .takes_value(true))
        .arg(Arg::with_name("threads")
            .long("threads")
            .value_name("INT")
            .takes_value(true))
        .arg(Arg::with_name("num_rules")
            .long("num_rules")
            .value_name("INT")
            .takes_value(true))
        .arg(Arg::with_name("num_def")
            .long("num_def")
            .value_name("INT")
            .takes_value(true))
        .arg(Arg::with_name("num_pred")
            .long("num_pred")
            .value_name("INT")
            .takes_value(true))
        .arg(Arg::with_name("num_events")
            .long("num_events")
            .value_name("INT")
            .takes_value(true))
        .arg(Arg::with_name("each_prob")
            .long("each_prob")
            .value_name("FLOAT")
            .takes_value(true))
        .arg(Arg::with_name("first_prob")
            .long("first_prob")
            .value_name("FLOAT")
            .takes_value(true))
        .arg(Arg::with_name("avg_win")
            .long("avg_win")
            .value_name("INT")
            .takes_value(true))
        .arg(Arg::with_name("consuming").long("consuming"))
        .arg(Arg::with_name("queue_len")
            .long("queue_len")
            .value_name("INT")
            .takes_value(true))
        .arg(Arg::with_name("evts_per_sec")
            .long("evts_per_sec")
            .value_name("INT")
            .takes_value(true))
        .arg(Arg::with_name("db_name")
            .long("db_name")
            .value_name("FILE")
            .takes_value(true))
        .arg(Arg::with_name("table_columns")
            .long("table_columns")
            .value_name("INT")
            .takes_value(true))
        .arg(Arg::with_name("table_rows")
            .long("table_rows")
            .value_name("INT")
            .takes_value(true))
        .arg(Arg::with_name("no_table_index").long("no_table_index"))
        .arg(Arg::with_name("cache_size")
            .long("cache_size")
            .value_name("INT")
            .takes_value(true))
        .arg(Arg::with_name("cache_ownership")
            .long("cache_ownership")
            .value_name("VAL")
            .help("SHARED|PER_PREDICATE")
            .takes_value(true))
        .arg(Arg::with_name("cache_type")
            .long("cache_type")
            .value_name("VAL")
            .help("DUMMY|LRU|LRU_SIZE|COLLISION|GDSF|GDS1|PERFECT")
            .takes_value(true))
        .arg(Arg::with_name("query_distribution")
            .long("query_distribution")
            .value_name("VAL")
            .help("Normal(sigma)|Exp(lambda)|Uniform(range)")
            .takes_value(true))
        .arg(Arg::with_name("matching_rows")
            .long("matching_rows")
            .value_name("INT")
            .takes_value(true))
        .arg(Arg::with_name("query_dependencies")
            .long("query_dependencies")
            .value_name("INT")
            .takes_value(true))
        .arg(Arg::with_name("static_prob")
            .long("static_prob")
            .value_name("FLOAT")
            .takes_value(true))
        .get_matches();

    let avg_win = matches.value_of("avg_win").map(|it| it.parse().unwrap()).unwrap_or(2000);
    let max_win = Duration::milliseconds(avg_win + 1000 as i64);
    let min_win = Duration::milliseconds(avg_win - 1000 as i64);

    let cfg = Config {
        seed: matches.value_of("seed").map(|it| it.parse().unwrap()).unwrap_or(rand::random()),
        threads: matches.value_of("threads")
            .map(|it| it.parse().unwrap())
            .unwrap_or(num_cpus::get()),
        num_rules: matches.value_of("num_rules").map(|it| it.parse().unwrap()).unwrap_or(1000),
        num_def: matches.value_of("num_def").map(|it| it.parse().unwrap()).unwrap_or(100),
        num_pred: matches.value_of("num_pred").map(|it| it.parse().unwrap()).unwrap_or(3),
        num_events: matches.value_of("num_events").map(|it| it.parse().unwrap()).unwrap_or(50_000),
        each_prob: matches.value_of("each_prob").map(|it| it.parse().unwrap()).unwrap_or(1.0),
        first_prob: matches.value_of("first_prob").map(|it| it.parse().unwrap()).unwrap_or(0.0),
        min_win: min_win,
        max_win: max_win,
        consuming: matches.is_present("consuming"),
        queue_len: matches.value_of("queue_len").map(|it| it.parse().unwrap()).unwrap_or(250),
        evts_per_sec: matches.value_of("evts_per_sec")
            .map(|it| it.parse().unwrap())
            .unwrap_or(3_000),
        db_name: matches.value_of("db_name").unwrap_or("./database.db").to_owned(),
        table_columns: matches.value_of("table_columns").map(|it| it.parse().unwrap()).unwrap_or(1),
        table_rows: matches.value_of("table_rows").map(|it| it.parse().unwrap()).unwrap_or(100_000),
        table_indexed: !matches.is_present("no_table_index"),
        cache_size: matches.value_of("cache_size").map(|it| it.parse().unwrap()).unwrap_or(250),
        cache_ownership: matches.value_of("cache_ownership")
            .map(|it| {
                match it {
                    "SHARED" => CacheOwnership::Shared,
                    "PER_PREDICATE" => CacheOwnership::PerPredicate,
                    _ => panic!("Unexpected cache ownership"),
                }
            })
            .unwrap_or(CacheOwnership::Shared),
        cache_type: matches.value_of("cache_type")
            .map(|it| {
                match it {
                    "DUMMY" => CacheType::Dummy,
                    "LRU" => CacheType::Lru,
                    "COLLISION" => CacheType::Collision,
                    "LRU_SIZE" => CacheType::LruSize,
                    "GDSF" => CacheType::Gdfs,
                    "GDS1" => CacheType::Gdf1,
                    "PERFECT" => CacheType::Perfect,
                    _ => panic!("Unexpected cache type"),
                }
            })
            .unwrap_or(CacheType::Lru),
        query_distribution: matches.value_of("query_distribution")
            .map(|it| {
                let cap =
                    Regex::new("(Normal|Exp|Uniform)\\((.+)\\)").unwrap().captures(it).unwrap();
                let par: f64 = cap.at(2).unwrap().parse().unwrap();
                match cap.at(1).unwrap() {
                    "Normal" => QueryDistribution::Normal(Normal::new(0.0, par)),
                    "Exp" => QueryDistribution::Exp(Exp::new(par)),
                    "Uniform" => {
                        QueryDistribution::Uniform(Range::new((-par / 2.0) as i64,
                                                              (par / 2.0) as i64))
                    }
                    _ => panic!("Unexpected distribution type"),
                }
            })
            .unwrap_or(QueryDistribution::Normal(Normal::new(0.0, 30.0))),
        matching_rows: matches.value_of("matching_rows")
            .map(|it| it.parse().unwrap())
            .unwrap_or(10),
        query_dependencies: matches.value_of("query_dependencies")
            .map(|it| it.parse().unwrap())
            .unwrap_or(1),
        static_prob: matches.value_of("static_prob").map(|it| it.parse().unwrap()).unwrap_or(0.2),
    };

    println!("{:?}", &cfg);

    execute_bench(&mut StdRng::from_seed(&[cfg.seed]), &cfg);
}
