use std::io::Error as IoError;
use std::io::ErrorKind as IoErrorKind;

#[derive(Debug)]
pub enum ErrorType {
    MissingArgument,
    FileAccessError(IoErrorKind),
}

impl From<IoError> for ErrorType {
    fn from(io: IoError) -> Self {
        ErrorType::FileAccessError(io.kind())
    }
}
