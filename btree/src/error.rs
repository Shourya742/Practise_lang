pub enum Error {
    KeyNotFound,
    KeyAlreadyExist,
    UnexpectedError,
    KeyOverflowError,
    ValueOverflowError,
    TryFromSliceError(&'static str),
    UTF8Error
}

impl From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Self {
        Error::UnexpectedError
    }
}