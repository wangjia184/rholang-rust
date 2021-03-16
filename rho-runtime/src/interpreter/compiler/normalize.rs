use super::context::{ SourcePosition, ProcVisitInputs, ProcVisitOutputs };
use super::bnfc;

pub struct NormalizeResult {
    // warning messages
    pub warnings : Vec<(SourcePosition, String)>,

    // error messages
    pub errors : Vec<(SourcePosition, String)>,
}

impl NormalizeResult {
    fn normalize_proc(&mut self, p : bnfc::Proc, input : ProcVisitInputs)  {

    }
}
