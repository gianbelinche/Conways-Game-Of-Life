use core::fmt;

pub enum CustomError {
    IncorrectNumberOfArguments,
    WrongState,
    MissingLines,
    FileNotFound,
    IncorrectFormat,
    BadMilliseconds,
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self {
            CustomError::IncorrectNumberOfArguments => "Incorrect number of arguments".to_string(),
            CustomError::WrongState => "Incorrect CSV format: state not 0 nor 1".to_string(),
            CustomError::MissingLines => "Incorrect CSV format: missing line(s)".to_string(),
            CustomError::FileNotFound => "File not found".to_string(),
            CustomError::IncorrectFormat => "Incorrect CSV format".to_string(),
            CustomError::BadMilliseconds => "Incorrect milliseconds format".to_string(),
        };
        write!(f, "{}", message)
    }
}
