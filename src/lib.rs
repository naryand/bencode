// bencode subfolder and item enum implemenation
use std::fmt::{self, Display};

use serde::{de, ser};

pub mod decode;
pub mod encode;

#[derive(Debug)]
pub enum Error {
    Message(String),
    Unimplemented,
    Overflow,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        match self {
            Error::Message(msg) => f.write_str(msg),
            Error::Unimplemented => f.write_str("Primitive is unimplemented"),
            Error::Overflow => f.write_str("Integer overflow"),
        }
    }
}

impl std::error::Error for Error {}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}
impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}
