use std::{convert::TryFrom, fmt};
use crate::{err, lisp};

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Symbol(lisp::Symbol),
    List(lisp::List),
    Fixnum(lisp::Fixnum),
    Nil,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Symbol(s) => fmt::Display::fmt(s, f),
            Object::List(l) => fmt::Display::fmt(l, f),
            Object::Fixnum(i) => write!(f, "{:#x}", i),
            Object::Nil => write!(f, "nil"),
        }
    }
}

impl Object {
    /// TODO: fix this to actually do a shallow copy
    pub fn shallow_copy(&self) -> Self {
        self.clone()
    }
}

// these macros take their arg as a token tree, instead of a type,
// so that they can use the same identifier as an `Object` enum
// discriminant and a `lisp` module member
macro_rules! derive_try_from {
    ($ty:tt) => {
        impl TryFrom<Object> for lisp::$ty {
            type Error = err::TypeError;
            fn try_from(obj: Object) -> Result<Self, Self::Error> {
                if let Object::$ty(it) = obj {
                    Ok(it)
                } else {
                    Err(err::TypeError {})
                }
            }
        }
    };
    ($($ty:tt)*) => {
        $(derive_from!($ty);)*
    };
}

macro_rules! derive_from {
    ($ty:tt) => {
        impl From<lisp::$ty> for Object {
            fn from(it: lisp::$ty) -> Self {
                Object::$ty(it)
            }
        }
    };
    ($($ty:tt)*) => {
        $(derive_from!($ty);)*
    };
}

derive_from!(Symbol Fixnum List);

derive_try_from!(Fixnum);
derive_try_from!(List);
derive_try_from!(Symbol);
