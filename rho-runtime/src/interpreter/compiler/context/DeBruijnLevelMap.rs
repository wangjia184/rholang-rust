

// free variable's level map
#[derive(Debug, Clone)]
pub struct LevelContext {
    pub level : i32,
    pub var_sort : VarSort,
    pub source_position : Rc<SourcePosition>,
}




// A structure to keep track of free variables, which are assigned DeBruijn levels (0 based).
#[derive(Debug, Clone)]
pub struct DeBruijnLevelMap {
    // The DeBruijn level assigned to the next variable name added to the map.
    pub next_level : i32, 

    // A map of names to DeBruijn levels.
    level_bindings : HashMap<RcString, Rc<LevelContext>>, 

    // A list of the positions of _ patterns.
    wildcards : Rc<Vec<SourcePosition>>,

    // A list of the positions of logical connectives.
    connectives : Rc<Vec<(Connective, SourcePosition)>>
}

impl DeBruijnLevelMap {
    // create an empty instance
    pub fn empty() -> DeBruijnLevelMap {
        DeBruijnLevelMap {
            next_level : 0,
            level_bindings : HashMap::new(),
            wildcards : Rc::new(Vec::new()),
            connectives : Rc::new(Vec::new()),
        }
    }
    
    pub fn get(&self, name: &str) -> Option<Rc<LevelContext>> {
        self.level_bindings.get(name).map(|b| b.clone())
    }

    // create a new DeBruijnLevelMap to store the binding
    pub fn put(&self, binding : BoundVariable) -> DeBruijnLevelMap {
        let mut new_bindings = self.level_bindings.clone();
        let ctx = Rc::new(LevelContext {
            level : self.next_level,
            var_sort : binding.1,
            source_position : Rc::new(binding.2),
        });
        new_bindings.insert( RcString::from(binding.0), ctx);
        DeBruijnLevelMap {
            next_level : self.next_level + 1,
            level_bindings : new_bindings,
            wildcards : self.wildcards.clone(),
            connectives : self.connectives.clone(),
        }
    }

    // create a new DeBruijnLevelMap and add a new wildcard binding
    pub fn add_wildcard(&self, source_position : SourcePosition) -> DeBruijnLevelMap{
        let mut wildcards : Vec<SourcePosition> = (*self.wildcards).clone();
        wildcards.push(source_position);
        DeBruijnLevelMap {
            next_level : self.next_level,
            level_bindings : self.level_bindings.clone(),
            wildcards : Rc::new(wildcards),
            connectives : self.connectives.clone(),
        }
    }
}