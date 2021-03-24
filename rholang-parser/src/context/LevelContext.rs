// free variable's level map
#[derive(Debug, Clone)]
pub struct LevelContext {
    pub level : i32,
    pub var_sort : VarSort,
    pub source_position : Rc<SourcePosition>,
}

