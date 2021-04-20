use std::sync::Arc;
use std::sync::atomic::{Ordering};


use super::*;
use async_trait::async_trait;
use tokio::task;

use model::*;

mod substitute;
pub mod eval_par;
mod eval_send;
mod eval_receive;
mod eval_expression;
mod environment;

#[cfg(test)] mod eval_receive_test;


use eval_expression::AsyncParExpressionEvaluator;
use substitute::*;
pub use environment::*;

use crate::storage::Storage;

#[async_trait]
pub trait AsyncEvaluator<S> where S : Storage + std::marker::Send + std::marker::Sync {
    async fn evaluate(&mut self, context : &Arc<InterpreterContext<S>>, env : &Env) -> Result<(), ExecutionError>;
}









impl<S : Storage + std::marker::Send + std::marker::Sync + 'static> InterpreterContext<S> {

    fn spawn_evaluation<T>(self : &Arc<Self>, t : T, env : &Env) -> tokio::task::JoinHandle<Result<(), ExecutionError>>
        where T : AsyncEvaluator<S> + std::marker::Send + 'static
    {
        let cloned_context = self.clone();
        let cloned_env = env.clone();
        task::spawn( async move {
            let mut evaluator = t;
            let reference = &mut evaluator;
            if let Err(err) = reference.evaluate(&cloned_context, &cloned_env).await {
                if err.kind != ExecutionErrorKind::Aborted as i32 {
                    cloned_context.aborted.store(true, Ordering::Relaxed);
                    cloned_context.errors.push(err.clone());
                }
                return Err(err);
            }
            Ok(())
        })
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
        // TODO : we need avoid async on handle_comm_events to reduce stack depth
        self.handle_comm_events(self.storage.produce(channel, data, persistent).await ).await;
    }

    #[inline]
    async fn consume(self : &Arc<Self>, binds : Vec<(BindPattern, Par)>,body : ParWithRandom, persistent : bool, peek : bool) {
        // TODO : we need avoid async on handle_comm_events to reduce stack depth
        self.handle_comm_events(self.storage.consume(binds, body, persistent, peek).await).await;
    }

    async fn handle_comm_events(self : &Arc<Self>, reply : Reply) {
        match reply {
            Reply::ParWithBody(par_with_rand, data_list) => {
                
                let pars : Vec<_> = data_list.into_iter().flat_map( |x| x.pars.into_iter() ).collect();
                let env = Env::<Par>::create(pars);
                match par_with_rand.body {
                    Some(par) => {
                        self.spawn_evaluation(par, &env).await;
                    },
                    _ => {

                    }
                }
            },
            Reply::None => (),
        }
        
    }
}

