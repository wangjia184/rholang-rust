
// Extends generated SourcePosition structure
impl SourcePosition {
    pub fn new (row : i32, col : i32, len : usize) -> SourcePosition {
        SourcePosition {
            row : row,
            col : col,
            len : len as i32,
        }
    }
}

impl fmt::Display for SourcePosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.row, self.col)
    }
}


