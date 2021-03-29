


use std::mem;

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
    pub fn clone_then_put(&self, binding : BoundVariable) -> Self {
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


    pub fn merge(&mut self, other : &Self ) -> Vec<(String, SourcePosition, SourcePosition)> {

        let shadowed = other.level_bindings.iter().fold( vec![], | mut shadowed, (key, value)| {
            match **value {
                LevelContext { ref level, ref var_sort, ref source_position } => {
                    if let Some(existing_ctx) = self.level_bindings.insert( key.clone(), Rc::new(LevelContext{
                        level : level + self.next_level,  
                        var_sort : *var_sort,
                        source_position : source_position.clone()
                    }) ) {
                        shadowed.insert(0, 
                            ( 
                                (**key).clone(), 
                                (**source_position).clone(),
                                (*(*existing_ctx).source_position).clone() 
                            )  
                        );
                    }
                }
            };
            shadowed
        });
        
        self.next_level = self.next_level + other.next_level;

        let mut wildcards = (*self.wildcards).clone();
        wildcards.extend((*other.wildcards).clone());
        drop(mem::replace(&mut self.wildcards, Rc::new(wildcards) )); // may change to Rc<RecCell<>> with interior mutability

        let mut connectives = (*self.connectives).clone();
        connectives.extend((*other.connectives).clone());
        drop(mem::replace(&mut self.connectives, Rc::new(connectives) ));

        shadowed
    }

    pub fn clone_then_merge(&self, other : &Self ) -> (Self, Vec<(String, Rc<SourcePosition>)>) {

        let init_acc = ( self.level_bindings.clone(), vec![] );

        let (lever_bindings, shadowed) = other.level_bindings.iter().fold(init_acc, | (mut lever_bindings, mut shadowed), (key, value)| {
            match **value {
                LevelContext { ref level, ref var_sort, ref source_position } => {
                    lever_bindings.insert( key.clone(), Rc::new(LevelContext{
                        level : level + self.next_level,  
                        var_sort : *var_sort,
                        source_position : source_position.clone()
                    }) );
                    if self.level_bindings.contains_key(key) {
                        shadowed.insert(0, ( (**key).clone(), (*source_position).clone() )  );
                    }
                }
            };
            (lever_bindings, shadowed)
        });

        let mut wildcards = (*self.wildcards).clone();
        wildcards.extend((*other.wildcards).clone());
        let mut connectives = (*self.connectives).clone();
        connectives.extend((*other.connectives).clone());
        (
            DeBruijnLevelMap {
                next_level : self.next_level + other.next_level,
                level_bindings : lever_bindings,
                wildcards : Rc::new(wildcards),
                connectives : Rc::new(connectives)
            }
            ,
            shadowed
        )
    }


    // create a new DeBruijnLevelMap and add a new wildcard binding
    pub fn clone_then_add_wildcard(&self, source_position : SourcePosition) -> Self{
        let mut wildcards : Vec<SourcePosition> = (*self.wildcards).clone();
        wildcards.push(source_position);
        DeBruijnLevelMap {
            next_level : self.next_level,
            level_bindings : self.level_bindings.clone(),
            wildcards : Rc::new(wildcards),
            connectives : self.connectives.clone(),
        }
    }

    pub fn count_no_wildcards(&self) -> i32 {
        self.next_level
    }
}