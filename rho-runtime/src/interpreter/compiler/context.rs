

use std::fmt;
use std::collections::HashMap;
use std::rc::Rc;

use super::super::super::model::rho_types::*;

// RcString is a wrapper of Rc<String>
// to reuse same String across levels of DeBruijnIndexMap
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct RcString(Rc<String>);

impl From<String> for RcString {
    fn from(s: String) -> Self {
        RcString(Rc::new(s))
    }
}

impl std::borrow::Borrow<str> for RcString {
    fn borrow(&self) -> &str {
        return &self.0;
    }
}



// Represents code location in source
#[derive(Debug)]
pub struct SourcePosition {
    pub row : u32,
    pub column : u32,
}
impl fmt::Display for SourcePosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.row, self.column)
    }
}

// It is either a `name` or a `process`
#[derive(Debug)]
pub enum VarSort {
    Process,
    Name,
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


// bound variable information
#[derive(Debug)]
pub struct IndexContext {
    index : u32,
    var_sort : VarSort,
    source_position : SourcePosition,
}

pub type BoundVariable = (String, VarSort, SourcePosition);



//  A structure to keep track of bound variables. The internal environment is the same as
//  DeBruijnLevelMap, but here we calculate the correct index on get. This way, we don't have to
//  reindex the map.
pub struct DeBruijnIndexMap {
    // The DeBruijn index assigned to the next variable added to the map.
    next_index : u32,

    // A map of names to DeBruijn indices.
    index_bindings : HashMap<RcString, Rc<IndexContext>>, 
}



impl DeBruijnIndexMap {

    // create an empty DeBruijnIndexMap
    pub fn empty() -> DeBruijnIndexMap {
        DeBruijnIndexMap{ 
            next_index : 0,
            index_bindings : HashMap::new()
        }
    }

    pub fn get(&self, name : &str) -> Option<Rc<IndexContext>> {
        self.index_bindings.get(name).map( |v| {
            v.clone()
        })
    }

    // create a new level basing on current level
    pub fn create(&mut self, bindings : Vec<BoundVariable>) -> DeBruijnIndexMap {
        let mut index_map = self.index_bindings.clone(); // clone 
        let next_index = self.next_index + bindings.len() as u32;

        let _overlapped_vars : Vec<_> = bindings
            .into_iter()
            .enumerate()
            .filter_map( |(idx, b)| {
                let ctx = Rc::new(IndexContext{
                    index : self.next_index + idx as u32,
                    var_sort : b.1,
                    source_position : b.2
                });
                index_map.insert(RcString::from(b.0), ctx.clone())
                    .map(|original_ctx| (original_ctx, ctx) )
            })
            .collect();
        // `overlapped_vars` contains the variable overwrites the parent level
        // [(IndexContext { index: 2, var_sort: Name, source_position: SourcePosition { row: 1, column: 8 } }, 
        //   IndexContext { index: 3, var_sort: Name, source_position: SourcePosition { row: 2, column: 9 } })]
        // TODO : should raise warning

        DeBruijnIndexMap{
            next_index : next_index,
            index_bindings : index_map,
        }
    }

}


#[test]
fn test_index_map() {
    // Suppose we have following rholang code
    // new a1, a2, a3 in {
    //     new b1, b2, a3 in {
    //         Nil
    //     }
    // }
    let mut level1 = DeBruijnIndexMap::empty().create(vec![
        ("a1".to_string(), VarSort::Name, SourcePosition{ row : 1, column : 2 } ),
        ("a2".to_string(), VarSort::Name, SourcePosition{ row : 1, column : 5 } ),
        ("a3".to_string(), VarSort::Name, SourcePosition{ row : 1, column : 8 } ),
    ]);
    let level2 = level1.create(vec![
        ("b1".to_string(), VarSort::Name, SourcePosition{ row : 2, column : 3 } ),
        ("b2".to_string(), VarSort::Name, SourcePosition{ row : 2, column : 6 } ),
        ("a3".to_string(), VarSort::Name, SourcePosition{ row : 2, column : 9 } ),
    ]);

    match level1.get("a1") {
        None => assert!(false),
        Some(x) => assert_eq!(x.index, 0),
    };
    match level1.get("a2") {
        None => assert!(false),
        Some(x) => assert_eq!(x.index, 1),
    };
    match level1.get("a3") {
        None => assert!(false),
        Some(x) => assert_eq!(x.index, 2),
    };
    assert!(level1.get("b1").is_none());
    assert!(level1.get("b2").is_none());

    match level2.get("a1") {
        None => assert!(false),
        Some(x) => assert_eq!(x.index, 0),
    };
    match level2.get("a2") {
        None => assert!(false),
        Some(x) => assert_eq!(x.index, 1),
    };
    match level2.get("b1") {
        None => assert!(false),
        Some(x) => assert_eq!(x.index, 3),
    };
    match level2.get("b2") {
        None => assert!(false),
        Some(x) => assert_eq!(x.index, 4),
    };
    match level2.get("a3") {
        None => assert!(false),
        Some(x) => assert_eq!(x.index, 5),
    };
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

// Input data to the normalizer
pub struct ProcVisitInputs {
    pub par : Par,
    pub env : IndexMapChain,
    pub known_free : DeBruijnLevelMap,
}

pub struct ProcVisitOutputs {
    pub par : Par,
    pub known_free : DeBruijnLevelMap,
}

