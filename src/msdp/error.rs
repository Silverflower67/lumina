use serde::{ser,de};
use std::fmt::{self, Display};

pub type Result<T> = std::result::Result<T,Error>;

#[derive(Debug)]
pub enum Error {
    Message(String),
    ExpectedArrayStart,
    ExpectedVal,
    ExpectedArrayEnd,
    ExpectedVar,
    ExpectedMapStart,
    ExpectedMapEnd,
    ExpectedMSDP,
}

impl ser::Error for Error {
    fn custom<T>(msg:T) ->Self
       where T:Display {
          Error::Message(msg.to_string())
       }
}

impl de::Error for Error {
    fn custom<T>(msg:T) ->Self
       where T:Display {
           Error::Message(msg.to_string())
       }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
           match self {
               Error::Message(s) => f.write_str(s),
               Error::ExpectedArrayStart => f.write_str("Expected MSDP array start"),
               Error::ExpectedVal => f.write_str("Expected MSDP Val"),
               Error::ExpectedArrayEnd => f.write_str("Expected MSDP array end"),
               Error::ExpectedVar => f.write_str("Expected MSDP Var"),
               Error::ExpectedMapStart => f.write_str("Expected MSDP map start"),
               Error::ExpectedMapEnd => f.write_str("Expected MSDP map end"),
               Error::ExpectedMSDP => f.write_str("Expected valid MSDP data")
           }
       }
}

impl std::error::Error for Error {}
