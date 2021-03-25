// A structure for keeping track of bound variables. Every time we go under a binding construct
// (e.g. match or receive), we add a fresh index map to the top of the chain. For a language like
// Java, each index map would represent a method's local variables
#[derive(Debug, Clone)]
pub struct IndexMapChain {
    chain : Vec<DeBruijnIndexMap>,
}

impl IndexMapChain {
    pub fn empty() -> IndexMapChain {
        IndexMapChain {
            chain : vec!(DeBruijnIndexMap::empty())
        }
    }

    pub fn get(&self, name : &str) -> Option<IndexContext> {
        self.chain[0].get(name)
    }

    pub fn put_head(&mut self, binding : BoundVariable) {
        self.chain[0] = self.chain[0].clone_then_put(vec![binding]);
    }

    pub fn add_bindings_to_head(mut self, bindings : Vec<BoundVariable>) -> IndexMapChain {
        self.chain[0] = self.chain[0].clone_then_put(bindings);
        self
    }

    pub fn count(&self) -> i32 { self.chain[0].count() }

    pub fn depth(&self) -> i32 { self.chain.len() as i32 - 1 }
}
