// A structure for keeping track of bound variables. Every time we go under a binding construct
// (e.g. match or receive), we add a fresh index map to the top of the chain. For a language like
// Java, each index map would represent a method's local variables.
pub struct IndexMapChain {
    chain : Vec<DeBruijnIndexMap>,
}

impl IndexMapChain {
    pub fn empty() -> IndexMapChain {
        IndexMapChain {
            chain : vec!(DeBruijnIndexMap::empty())
        }
    }
}
