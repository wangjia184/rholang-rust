

use std::fmt;
use std::collections::HashMap;
use std::rc::Rc;

use super::super::super::model::rho_types::*;

pub struct SourcePosition {
    pub row : u32,
    pub column : u32,
}
impl fmt::Display for SourcePosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.row, self.column)
    }
}

// free variable's level map
pub struct LevelContext {
    pub level : u32,
    pub source_position : Rc<SourcePosition>,
}

// A structure to keep track of free variables, which are assigned DeBruijn levels (0 based).
pub struct DeBruijnLevelMap {
    // The DeBruijn level assigned to the next variable name added to the map.
    next_level : u32, 

    // A map of names to DeBruijn levels.
    level_bindings : HashMap<String, LevelContext>, 

    // A list of the positions of _ patterns.
    wildcards : Vec<SourcePosition>,

    // A list of the positions of logical connectives.
    //connectives : Vec<(ConnectiveInstance, SourcePosition)>
}

impl DeBruijnLevelMap {
    // create an empty instance
    pub fn empty() -> DeBruijnLevelMap {
        DeBruijnLevelMap {
            next_level : 0,
            level_bindings : HashMap::new(),
            wildcards : Vec::new(),
        }
    }
}


pub struct IndexContxt {
    index : u32,
    source_position : SourcePosition,
}


//  A structure to keep track of bound variables. The internal environment is the same as
//  DeBruijnLevelMap, but here we calculate the correct index on get. This way, we don't have to
//  reindex the map.
pub struct DeBruijnIndexMap {
    // The DeBruijn index assigned to the next variable added to the map.
    next_level : u32,

    // A map of names to DeBruijn indices.
    index_bindings : HashMap<String, IndexContxt>, 
}

impl DeBruijnIndexMap {

    // create an empty DeBruijnIndexMap
    pub fn empty() -> DeBruijnIndexMap {
        DeBruijnIndexMap{ 
            next_level : 0,
            index_bindings : HashMap::new()
        }
    }
}

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

#[test]
pub fn test1() {
    let _ = IndexMapChain::empty();

}