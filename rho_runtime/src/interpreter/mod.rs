use std::{sync::atomic::{AtomicBool}, thread::JoinHandle};
use crossbeam::queue::SegQueue;


mod reduce;
pub use reduce::*;

use super::storage::*;
use model::*;

pub struct InterpreterContext<S> where S : Storage + std::marker::Send + std::marker::Sync {
    storage : S,
    aborted : AtomicBool,
    errors : SegQueue<ExecutionError>,
    join_handles : SegQueue<JoinHandle<()>>,
}

impl<S:Storage + std::marker::Send + std::marker::Sync> From<S> for InterpreterContext<S> {
    fn from(storage: S) -> Self {
        Self {
            storage : storage,
            aborted : AtomicBool::default(),
            errors : SegQueue::default(),
            join_handles : SegQueue::default(),
        }
    }
}