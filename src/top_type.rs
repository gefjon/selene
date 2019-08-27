use std::{convert, fmt};
use crate::{err, lisp};

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Symbol(lisp::Symbol),
    List(lisp::List),
    Integer(lisp::Fixnum),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Symbol(s) => fmt::Display::fmt(s, f),
            Object::List(l) => fmt::Display::fmt(l, f),
            Object::Integer(i) => write!(f, "{:#x}", i),
        }
    }
}

impl From<lisp::Symbol> for Object {
    fn from(s: lisp::Symbol) -> Object {
        Object::Symbol(s)
    }
}

impl From<lisp::List> for Object {
    fn from(l: lisp::List) -> Object {
        Object::List(l)
    }
}

impl From<lisp::Fixnum> for Object {
    fn from(n: lisp::Fixnum) -> Object {
        Object::Integer(n)
    }
}

impl Object {
    /// TODO: fix this to actually do a shallow copy
    pub fn shallow_copy(&self) -> Self {
        self.clone()
    }
}

impl convert::TryFrom<lisp::Object> for lisp::Fixnum {
    type Error = err::TypeError;
    fn try_from(obj: lisp::Object) -> Result<Self, Self::Error> {
        if let lisp::Object::Integer(i) = obj {
            Ok(i)
        } else {
            Err(err::TypeError {})
        }
    }
}
