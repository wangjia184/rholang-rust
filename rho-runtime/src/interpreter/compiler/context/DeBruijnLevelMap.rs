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