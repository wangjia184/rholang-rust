use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::sync::atomic::{ AtomicI32, Ordering };


use crossbeam::{queue::SegQueue, sync::ShardedLock};
use rustc_hash::FxHashMap;
use tokio::task;
use tokio::sync::Notify;


mod reduce;
pub use reduce::*;
pub mod system_process;
mod hash_rand; 
use hash_rand::HashRand;

use super::storage::*;
use model::*;

#[derive(Default)]
struct AsynWaitGroup {
    notify : Notify,
    count : AtomicI32,
}

impl AsynWaitGroup {
    #[inline]
    fn acquire(&self) {
        self.count.fetch_add(1, Ordering::Relaxed);
    }

    #[inline]
    fn release(&self) {
        if self.count.fetch_sub(1, Ordering::Relaxed) <= 1 /* this was the last one */ {
            self.notify.notify_one();
        }
    }
}

pub struct InterpreterContext<S> where S : Storage + std::marker::Send + std::marker::Sync {
    storage : S,
    aborted : AtomicBool,
    errors : SegQueue<ExecutionError>,
    wait_group : AsynWaitGroup,
    urn_map : ShardedLock<FxHashMap<String, Par>>,
}

impl<S:Storage + std::marker::Send + std::marker::Sync> From<S> for InterpreterContext<S> {
    fn from(storage: S) -> Self {
        Self {
            storage : storage,
            aborted : AtomicBool::default(),
            errors : SegQueue::new(),
            wait_group : AsynWaitGroup::default(),
            urn_map : ShardedLock::new(system_process::get_map()),
        }
    }
}




impl<S : Storage + std::marker::Send + std::marker::Sync + 'static> InterpreterContext<S> {

    // this is the entrypoint, it must not be called by interpretered pars
    pub async fn evaludate(self : &Arc<Self>, mut par : Par) {
        let env = Env::<Par>::default();
        if let Err(e) = par.evaluate(&self, &env).await
        {
            self.push_error(e);
        }
        self.wait_group.notify.notified().await
    }

    #[inline]
    fn push_error(self : &Arc<Self>, err : ExecutionError) {
        self.aborted.store(true, Ordering::Relaxed);
        if err.kind != ExecutionErrorKind::Aborted as i32 {
            self.errors.push(err.clone());
        }
    }

    #[inline]
    fn spawn_evaluation<T>(self : &Arc<Self>, t : T, env : &Env)
        where T : AsyncEvaluator<S> + std::marker::Send + 'static
    {
        let cloned_context = self.clone();
        let cloned_env = env.clone();
        self.wait_group.acquire();
        task::spawn( async move {
            let mut evaluator = t;
            let reference = &mut evaluator;
            if let Err(e) = reference.evaluate(&cloned_context, &cloned_env).await {
                cloned_context.push_error(e);
            }
            cloned_context.wait_group.release();
        });
    }

    

    
    #[inline]
    pub fn may_raise_aborted_error(&self) -> Result<(), ExecutionError> {
        if self.aborted.load(Ordering::Relaxed) {
            Err(ExecutionError{
                kind : ExecutionErrorKind::Aborted as i32,
                message : "aborted".to_string(),
            })
        } else {
            Ok(())
        }
    }


    
    #[inline]
    async fn produce(self : &Arc<Self>, channel : Par, data : ListParWithRandom, persistent : bool) {
        self.handle_comm_events(
            self.storage.produce(channel, data, persistent).await
        );
    }

    #[inline]
    async fn consume(self : &Arc<Self>, binds : Vec<(BindPattern, Par)>,body : ParWithRandom, persistent : bool, peek : bool) {
        self.handle_comm_events(
            self.storage.consume(binds, body, persistent, peek).await
        );
    }

    fn handle_comm_events(self : &Arc<Self>, reply : Reply) {
        match reply {
            Some(vector) => {
                for (continuation, data_list) in vector {
                    match continuation {
                        TaggedContinuation::ParBody(par_with_rand) => {
                            let pars = data_list.into_iter().flat_map( |x| x.pars.into_iter() ).collect();
                            let env = Env::<Par>::create(pars);
                            match par_with_rand.body {
                                Some(par) => {
                                    self.spawn_evaluation(par, &env);
                                },
                                _ => {

                                }
                            }
                        },
                        TaggedContinuation::Callback(func) => {
                            task::spawn_blocking(move ||func(data_list));
                        }
                    }
                }
            },
            None => (),
        }
        
    }
}
