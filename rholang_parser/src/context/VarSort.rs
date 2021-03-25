


// It is either a `name` or a `process`
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum VarSort {
    Process,
    Name,
}

impl fmt::Display for VarSort {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
        VarSort::Process => write!(f, "VarSort::Process"),
        VarSort::Name => write!(f, "VarSort::Name"),
        //_ => write!(f, "VarSort::{}", *self as i32),
       }
    }
}