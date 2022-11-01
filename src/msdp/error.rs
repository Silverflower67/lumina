use serde::{de, ser};
use std::fmt::{self, Display, format};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug,Clone)]
pub struct Nom {
    pub input: Vec<u8>,
    pub kind: nom::error::ErrorKind
}

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
    TrailingBytes,
    Eof,
    Parse(&'static str),
    Nom(Nom),
    MultiNom(Vec<Nom>)
}

impl ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Error::Message(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
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
            Error::ExpectedMSDP => f.write_str("Expected valid MSDP data"),
            Error::TrailingBytes => f.write_str("Trailing bytes"),
            Error::Eof => f.write_str("Unexpected EOF"),
            Error::Parse(s) => f.write_str(s),
            Error::Nom(Nom {input, kind}) => f.write_fmt(format_args!("{:?}: {:?}",input,kind)),
            Error::MultiNom(internal) => {
                let s = internal.iter().map(|n| Error::Nom(n.to_owned()).to_string()).collect::<Vec<_>>().join("\n");
                f.write_str(s.as_str())
            }
        }
    }
}

impl std::error::Error for Error {}

impl nom::error::ParseError<&[u8]> for Error {
    fn from_error_kind(input: &[u8], kind: nom::error::ErrorKind) -> Self {
        Error::Nom(Nom {input: input.to_owned(), kind})
    }

    fn append(input: &[u8], kind: nom::error::ErrorKind, other: Self) -> Self {
        match other {
            Self::Nom(internal) => Error::MultiNom(vec![Nom {input: input.to_owned(), kind},internal]),
            Self::MultiNom(internal) => {
                let mut new = internal;
                new.push(Nom {input: input.to_owned(), kind});
                Error::MultiNom(new)
            },
            _ => {
                let n = Nom {input: input.to_owned(), kind: nom::error::ErrorKind::Tag};
                Error::Nom(n)
            }
        }
    }
}