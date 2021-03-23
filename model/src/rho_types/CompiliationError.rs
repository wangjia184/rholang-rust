
// Extends the generated SyntaxError struct
impl CompiliationError {
    pub fn new_null_pointer(pointer_name : &str) -> Self {
        CompiliationError {
            kind : CompiliationErrorKind::NullPointer as i32,
            message : format!("Null pointer is detected when accessing `{}`", pointer_name),
            position : None,
            contra_position : None,
        }
    }

    pub fn new_unrecognized_token(token : u32, row : i32, column : i32) -> Self {
        CompiliationError {
            kind : CompiliationErrorKind::UnrecognizedToken as i32,
            message : format!("Unrecognized token `{}` at {}:{}", token, row, column),
            position : Some(SourcePosition{ row : row, col : column, len : 0 }),
            contra_position : None,
        }
    }

    pub fn new_unrecognized_data(name : &str, token : u32, row : i32, column : i32) -> Self {
        CompiliationError {
            kind : CompiliationErrorKind::UnrecognizedData as i32,
            message : format!("Unrecognized data {}={} at {}:{}", name, token, row, column),
            position : Some(SourcePosition{ row : row, col : column, len : 0 }),
            contra_position : None,
        }
    }

    pub fn new_utf8_error(err : &std::str::Utf8Error, row : i32, column : i32) -> Self {
        CompiliationError {
            kind : CompiliationErrorKind::Utf8Error as i32,
            message : format!("Invalid utf8 encoding at {}:{}. {}", row, column, err),
            position : Some(SourcePosition{ row : row, col : column, len : 0 }),
            contra_position : None,
        }
    }

    pub fn new_invalid_file_handle() -> Self {
        CompiliationError {
            kind : CompiliationErrorKind::InvalidFileHandle as i32,
            message : format!("Invalid file handle"),
            position : None,
            contra_position : None,
        }
    }

}







impl fmt::Display for CompiliationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}", &self.kind, &self.message)
    }
}

impl Error for CompiliationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

