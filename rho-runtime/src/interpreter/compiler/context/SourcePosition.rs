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