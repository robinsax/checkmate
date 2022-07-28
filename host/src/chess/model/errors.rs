use std::fmt;

#[derive(Debug)]
pub enum ValidationError {
    Parse{token: String}
}

impl fmt::Display for ValidationError {
    fn fmt(&self, dest: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::Parse{token} => write!(dest, "invalid: {}", token)
        }
    }
}
