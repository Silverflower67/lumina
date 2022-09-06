use serde_json::Error as JSONError;
use serde::{ser,de};
use std::fmt::{self,Display};
pub type Result<T> = std::result::Result<T,Error>;

#[derive(Debug)]
pub enum Error {
    Message(String),
    JSON(JSONError),
    ExpectedSB,
    ExpectedSE,
    ExpectedGMCP
}

impl ser::Error for Error {
    fn custom<T: Display>(msg:T) -> Self {
           Error::Message(msg.to_string())
       }
}

impl de::Error for Error {
    fn custom<T: Display>(msg:T) -> Self {
           Error::Message(msg.to_string())
       }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
           match self {
               Error::Message(msg) => f.write_str(msg),
               Error::JSON(err) => err.fmt(f),
               Error::ExpectedGMCP => f.write_str("Expected valid GMCP data"),
               Error::ExpectedSB => f.write_str("Expected IAC SB"),
               Error::ExpectedSE => f.write_str("Expected IAC SE")
           }
       }
}
impl std::error::Error for Error {}
