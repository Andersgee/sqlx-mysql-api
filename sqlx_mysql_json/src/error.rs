use std::fmt;

//#[derive(Debug)]
pub enum Error {
    Parameter(String),
    TupleType(String),
    Decode(String),
    Sqlx(String),
    SerdeJson(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Parameter(s) => write!(f, "{:?}", s),
            Error::TupleType(s) => write!(f, "{:?}", s),
            Error::Decode(s) => write!(f, "{:?}", s),
            Error::Sqlx(s) => write!(f, "{:?}", s),
            Error::SerdeJson(s) => write!(f, "{:?}", s),
        }
    }
}
