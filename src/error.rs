use std::{fmt, io};

#[derive(Debug)]
pub enum Error {
    Http(ureq::Error),
    IO(io::Error),
}

impl From<ureq::Error> for Error {
    fn from(e: ureq::Error) -> Self {
        Self::Http(e)
    }
}

impl std::error::Error for Error {}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::IO(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Http(e) => write!(f, "Http Error: {}", e),
            Self::IO(e) => write!(f, "{}", e),
        }
    }
}
