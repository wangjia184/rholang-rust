

use async_trait::async_trait;
use smallvec::{ SmallVec, smallvec };
use tokio::sync::oneshot;

use model::*;

mod coordinator;
mod transit;

use transit::*;
pub  use coordinator::*;

pub type ShortVector<T> = SmallVec<[T; 3]>;




pub type Reply = Option<ShortVector<(TaggedContinuation, ShortVector<ListParWithRandom>)>>;

pub type RustCallbacFunction = fn(ShortVector<ListParWithRandom>) -> ();

#[derive(Clone)]
pub enum TaggedContinuation {
    ParBody(ParWithRandom),
    Callback(RustCallbacFunction)
}



#[async_trait]
pub trait Storage { 
    async fn install(&self, channel : Par, bind_pattern : BindPattern, func : RustCallbacFunction) -> Reply;

    async fn produce(&self, channel : Par, data : ListParWithRandom, persistent : bool) -> Reply;

    async fn consume(&self, binds : Vec<(BindPattern, Par)>,body : ParWithRandom, persistent : bool, peek : bool) -> Reply;
}






