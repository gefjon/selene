use crate::lisp;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    UnknownCompilerForm(lisp::List),
    StackUnderflow,
    TypeError(TypeError),
}

#[derive(Clone, Debug)]
pub struct TypeError {}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error::Io(e)
    }
}

impl From<TypeError> for Error {
    fn from(e: TypeError) -> Self {
        Error::TypeError(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
