use std::fmt;

#[derive(Debug, PartialEq)]
pub enum ValidationError {
    Parse{token: String},
    InvalidState{token: String}
}

impl fmt::Display for ValidationError {
    fn fmt(&self, dest: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::Parse{token} => write!(dest, "invalid format: {}", token),
            ValidationError::InvalidState{token} => write!(dest, "invalid state: {}", token)
        }
    }
}
