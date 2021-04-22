use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::sync::atomic::Ordering;

use crossbeam::{queue::SegQueue, sync::ShardedLock};
use rustc_hash::FxHashMap;
use tokio::task::JoinHandle;
use tokio::task;

mod reduce;
pub use reduce::*;
pub mod system_process;
mod hash_rand; 
use hash_rand::HashRand;

use super::storage::*;
use model::*;

pub struct InterpreterContext<S> where S : Storage + std::marker::Send + std::marker::Sync {
    storage : S,
    aborted : AtomicBool,
    join_handles : SegQueue<JoinHandle<Result<(), ExecutionError>>>,
    urn_map : ShardedLock<FxHashMap<String, Par>>,
}

impl<S:Storage + std::marker::Send + std::marker::Sync> From<S> for InterpreterContext<S> {
    fn from(storage: S) -> Self {
        Self {
            storage : storage,
            aborted : AtomicBool::default(),
            join_handles : SegQueue::default(),
            urn_map : ShardedLock::new(system_process::get_map()),
        }
    }
}




impl<S : Storage + std::marker::Send + std::marker::Sync + 'static> InterpreterContext<S> {

    pub async fn evaludate(self : &Arc<Self>, mut par : Par) -> Vec<ExecutionError> {
        let env = Env::<Par>::default();
        let mut errors : Vec<ExecutionError> = Vec::new();
        par.evaluate(&self, &env).await.expect("par.evaluate(&self, &env) failed"); // should never fail
        while let Some(handle) = self.join_handles.pop() {
            let result = handle.await;
            match result {
                Ok(Err(err)) => {
                    if err.kind != ExecutionErrorKind::Aborted as i32 {
                        self.aborted.store(true, Ordering::Relaxed);
                        errors.push(err.clone());
                    }
                },
                Err(err) => panic!("JoinError occured in InterpreterContext::evaludate. {}", err),
                _ => (),
            }
        }
        errors
    }

    fn spawn_evaluation<T>(self : &Arc<Self>, t : T, env : &Env)
        where T : AsyncEvaluator<S> + std::marker::Send + 'static
    {
        let cloned_context = self.clone();
        let cloned_env = env.clone();
        self.join_handles.push(
            task::spawn( async move {
                let mut evaluator = t;
                let reference = &mut evaluator;
                reference.evaluate(&cloned_context, &cloned_env).await
            })
        );
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
            Some((TaggedContinuation::ParBody(par_with_rand), data_list)) => {
                let pars = data_list.into_iter().flat_map( |x| x.pars.into_iter() ).collect();
                let env = Env::<Par>::create(pars);
                match par_with_rand.body {
                    Some(par) => {
                        self.spawn_evaluation(par, &env);
                    },
                    _ => {

                    }
                }
            }
            Some((TaggedContinuation::Callback(func), data_list)) => {
                task::spawn_blocking(move ||func(data_list));
            }
            Reply::None => (),
        }
        
    }
}
