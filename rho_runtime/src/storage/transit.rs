
use std::rc::Rc;
use std::cell::RefCell;
use blake3::Hash;
use tokio::task::JoinHandle;
use rustc_hash::{FxHashMap, FxHashSet};
use tokio::sync::oneshot::Receiver;

use super::*;
use smallvec::SmallVec;

pub(super) type ShortVector<T> = SmallVec<[T; 5]>;

#[derive(Default, Debug)]
pub struct  Cargo{

    unmatched_produces : SmallVec<[ProduceTask; 5]>,
    unmatched_consumes : SmallVec<[ConsumeTask; 5]>,  // should build a tree to optimize the matcher
    unprocessed_produces : Option<ShortVector<ProduceTask>>,
    unprocessed_consumes : Option<ShortVector<ConsumeTask>>,
}

#[derive(Debug)]
pub(super) struct  TransitPort {
    pub(super) cargo : Option<Cargo>,
    pub(super) unprocessed_produces : Option<ShortVector<ProduceTask>>,
    pub(super) unprocessed_consumes : Option<ShortVector<ConsumeTask>>,
    pub(super) reference_count : FxHashMap<Hash, i32/* number of reference from consumes */>,
    pub(super) join_group : Option<Rc<RefCell<JoinGroup>>>,
    pub(super) signal : Option<Receiver<()>>,
}

impl Default for TransitPort {
    fn default() -> Self {
        Self {
            cargo : Some(Cargo::default()),
            unprocessed_produces : None,
            unprocessed_consumes : None,
            reference_count : FxHashMap::default(),
            join_group : None,
            signal : None,
        }
    }
}


#[derive(Debug)]
pub(super) struct  JoinGroup {
    pub(super) id : u64,
    pub(super) channel_set : FxHashSet<Hash>,
    pub(super) handle : Option<JoinHandle<()>>,
}




