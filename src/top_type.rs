use std::fmt;
use crate::lisp;

pub enum Object {
    Symbol(lisp::Symbol),
    List(lisp::List),
    Integer(i64),
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

impl From<i64> for Object {
    fn from(n: i64) -> Object {
        Object::Integer(n)
    }
}
