use std::convert::Into;

// Extends the generated ExecutionError struct
impl ExecutionError {
    pub fn new_invalid_expression<S>(message : S) -> Self where S : Into<String> {
        Self::new(ExecutionErrorKind::InvalidExpression, message)
    }

    pub fn new<S>(kind : ExecutionErrorKind, message : S) -> Self where S : Into<String> {
        Self {
            kind : kind as i32,
            message : message.into()
        }
    }

}







impl fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}", &self.kind, &self.message)
    }
}

impl Error for ExecutionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

