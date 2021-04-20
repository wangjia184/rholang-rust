

use async_trait::async_trait;
use smallvec::{ SmallVec, smallvec };
use tokio::sync::oneshot;

use model::*;

mod coordinator;
mod transit;

use transit::*;
pub  use coordinator::*;

type ShortVector<T> = SmallVec<[T; 3]>;

#[async_trait]
pub trait Storage { 
    async fn produce(&self, channel : Par, data : ListParWithRandom, persistent : bool) -> Reply;

    async fn consume(&self, binds : Vec<(BindPattern, Par)>,body : ParWithRandom, persistent : bool, peek : bool) -> Reply;
}






