




pub type BoundVariable = (String, VarSort, SourcePosition);



//  A structure to keep track of bound variables. The internal environment is the same as
//  DeBruijnLevelMap, but here we calculate the correct index on get. This way, we don't have to
//  reindex the map.
#[derive(Debug, Clone)]
pub struct DeBruijnIndexMap {
    // The DeBruijn index assigned to the next variable added to the map.
    next_index : i32,

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

    pub fn count(&self) -> i32 { self.next_index }

    pub fn get(&self, name : &str) -> Option<IndexContext> {
        self.index_bindings.get(name).map( |v| IndexContext{
            index : self.next_index - v.index - 1, // copy from scala source, dont understand now
            var_sort : v.var_sort,
            source_position : v.source_position.clone(),
        })
    }

    // create a new DeBruijnIndexMap basing on current level
    pub fn clone_then_put(&mut self, bindings : Vec<BoundVariable>) -> DeBruijnIndexMap {
        let mut index_map = self.index_bindings.clone(); // clone 
        let next_index = self.next_index + bindings.len() as i32;

        let _overlapped_vars : Vec<_> = bindings
            .into_iter()
            .enumerate()
            .filter_map( |(idx, b)| {
                let ctx = Rc::new(IndexContext{
                    index : self.next_index + idx as i32,
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






