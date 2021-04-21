use super::*;
use rustc_hash::FxHashMap;


lazy_static! {
    static ref HASHMAP: FxHashMap<String, Par> = {
        let mut map = FxHashMap::default();
        map.insert(StdoutContract::name(), StdoutContract::par());
        map
    };
}

pub fn get_map() -> FxHashMap<String, Par> {
    HASHMAP.clone()
}

pub trait SystemContract {
    fn name() -> String;
    fn par() -> Par;
    fn execute(par : Par);
}


pub struct StdoutContract {
}

impl SystemContract for StdoutContract {
    fn name() -> String { "rho:stdout".to_owned() }
    
    fn par() -> Par {
        let mut par = Par::default();
        par.unforgeables.push(
            {
                GUnforgeable {
                    unf_instance : Some(g_unforgeable::UnfInstance::GPrivateBody(
                        GPrivate {
                            id : HashRand::next(),
                        }
                    ))
                }
            }
        );
        par
    }

    fn execute(par : Par) {

    }
}