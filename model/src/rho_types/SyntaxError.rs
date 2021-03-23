
// Extends the generated SyntaxError struct
impl SyntaxError {
    pub fn new_unexpected_name_context(var_name : &str, source_position : SourcePosition, contra_source_position : SourcePosition) -> Self {
        SyntaxError {
            kind : SyntaxErrorKind::UnexpectedNameContext as i32,
            message : format!("Process variable `{}` is used as a name variable", var_name),
            position : Some(source_position),
            contra_position : Some(contra_source_position),
        }
    }

    pub fn new_integer_number_error(value : &str, source_position : SourcePosition) -> Self {
        SyntaxError {
            kind : SyntaxErrorKind::IntegerNumberError as i32,
            message : format!("Invalid integer number `{}`", value),
            position : Some(source_position),
            contra_position : None,
        }
    }

    pub fn new_unexpected_reuse_of_name_context_free(value : &str, source_position : SourcePosition, contra_source_position : SourcePosition) -> Self {
        SyntaxError {
            kind : SyntaxErrorKind::UnexpectedReuseOfNameContextFree as i32,
            message : format!("Free variable `{}` is used twice as a binder in name context", value),
            position : Some(source_position),
            contra_position : Some(contra_source_position),
        }
    }

    pub fn new_empty_uri(source_position : SourcePosition) -> Self {
        SyntaxError {
            kind : SyntaxErrorKind::EmptyUri as i32,
            message : format!("Empty uri"),
            position : Some(source_position),
            contra_position : None,
        }
    }
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.message)
    }
}

impl Error for SyntaxError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

