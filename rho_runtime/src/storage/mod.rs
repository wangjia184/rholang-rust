

use async_trait::async_trait;
use model::*;

mod dispatch;
mod hot_store;

pub  use dispatch::*;

#[async_trait]
pub trait Storage { 
    async fn produce(&self, channel : Par, data : ListParWithRandom, persist : bool);
}






