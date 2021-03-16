

// RcString is a wrapper of Rc<String>
// to reuse same String across levels of DeBruijnIndexMap
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct RcString(Rc<String>);

impl From<String> for RcString {
    fn from(s: String) -> Self {
        RcString(Rc::new(s))
    }
}

impl std::borrow::Borrow<str> for RcString {
    fn borrow(&self) -> &str {
        return &self.0;
    }
}

