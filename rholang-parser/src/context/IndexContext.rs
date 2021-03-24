// bound variable information
#[derive(Debug)]
pub struct IndexContext {
    pub index : i32,
    pub var_sort : VarSort,
    pub source_position : SourcePosition,
}
