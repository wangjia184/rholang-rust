mod substitute;
use std::sync::Arc;

use model::*;

mod reduce;



pub async fn test_reduce() {

    let mut par = Par::default();
    par.sends.push(Send{
        chan : None,
        data : Vec::<Par>::new(),
        locally_free : None,
        persistent : false,
        connective_used : false,
    });

    let reducer = Arc::new(reduce::DebruijnInterpreter{ });
    reducer.evaluate(par).await;
}