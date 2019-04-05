use std::fmt;
use std::io;

pub type Result = std::result::Result<(), Error>;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
}

impl Error {
    pub fn exit(self) {
        eprintln!("Shell terminated unexpectedly with error: {}", self);
        std::process::exit(-1);
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::Io(error)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Io(ref err) => err.fmt(f),
        }
    }
}
