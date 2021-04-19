

use async_trait::async_trait;
use model::*;

mod coordinator;
mod transit;

use transit::*;
pub  use coordinator::*;

#[async_trait]
pub trait Storage { 
    async fn produce(&self, channel : Par, data : ListParWithRandom, persistent : bool);

    async fn consume(&self, binds : Vec<(BindPattern, Par)>,body : ParWithRandom, persistent : bool, peek : bool);
}






