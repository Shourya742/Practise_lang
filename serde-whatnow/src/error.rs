use std::fmt::{self, Display, Formatter};

use serde::{de, ser};

pub type Result<T> = std::result::Result<T, Error>;



// This is bare-bones implementation. A real library would provide additional
// information in its error type, for example the line and column at which the
// error occured, the byte offset into the input, or the current key being
// processed
#[derive(Debug)]
pub enum Error {
    // One or more variants that can be created by data structures through the
    // `ser::Error` and `de::Error` traits,. For example the Serialize impl for
    // Mutex<T> might return an error because the mutex is poisoned, or the 
    // Derserialize impl for a struct may return an error because a required
    // field is missing.
    Message(String),
    // Zero or more variants that can be created directly by the serializer and Deserializer without going 
    // through `ser::Error` and `de::Error`. These are specifc to the format, in this case JSONy.
    Eof,
    Syntax,
    ExpectedBoolean,
    ExpectedInteger,
    ExpectedString,
    ExpectedNull,
    ExpectedArray,
    ExpectedArrayComma,
    ExpectedArrayEnd,
    ExpectedMap,
    ExpectedMapColon,
    ExpectedMapComma,
    ExpectedMapEnd,
    ExpectedEnum,
    TrailingCharacters
}

impl Display for Error {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Error::Message(msg) => formatter.write_str(&msg),
            _ => formatter.write_str("Something something")
        }
    }
}

impl std::error::Error for Error {}

impl ser::Error for Error {
    fn custom<T>(msg:T) -> Self where T:Display {
        Error::Message(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T>(msg:T) -> Self where T:Display {
        Error::Message(msg.to_string())
    }
}