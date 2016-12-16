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
use tesla::expressions::{BasicType, BinaryOperator, Expression, Value};
use tesla::predicates::{ConstrainedTuple, EventSelection, ParameterDeclaration, Predicate,PredicateType, Timing, TimingBound};
use trex::TRex;
use trex::stack::StackProvider;

use std::sync::{Mutex, Once, ONCE_INIT};
use std::{mem};

use conn_queues::{insert_queue, pop_queue, print_queue_status,remove_queue,init_queue};

// TODO: create the functions to convert from/to json data

// fn json_to_event_dec() -> {}
// fn json_to_rule_def() -> {}
// fn json_to_event() -> {}
