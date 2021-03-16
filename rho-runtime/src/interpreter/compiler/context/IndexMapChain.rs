// A structure for keeping track of bound variables. Every time we go under a binding construct
// (e.g. match or receive), we add a fresh index map to the top of the chain. For a language like
// Java, each index map would represent a method's local variables
#[derive(Debug)]
pub struct IndexMapChain {
    chain : Vec<DeBruijnIndexMap>,
}

impl IndexMapChain {
    pub fn empty() -> IndexMapChain {
        IndexMapChain {
            chain : vec!(DeBruijnIndexMap::empty())
        }
    }

    // this method creates a new IndexMapChain instance by replace currenct chain's head
    // same behaviour as IndexMapChain.put() in scala source
    pub fn clone_with_new_head(&self, bindings : Vec<BoundVariable>) -> IndexMapChain {
        let mut chain = self.chain.clone();
        chain[0] = chain[0].create(bindings);
        IndexMapChain {
            chain : chain,
        }
    }

    pub fn count(&self) -> u32 { self.chain[0].count() }
}
