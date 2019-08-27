use std::{fmt::{self, Write}, ops};

use crate::lisp;

#[derive(Debug, Clone, PartialEq)]
pub struct List {
    vec: Vec<lisp::Object>,
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_char('(')?;
        let mut iter = self.vec.iter().peekable();
        while let Some(item) = iter.next() {
            fmt::Display::fmt(item, f)?;
            if iter.peek().is_some() {
                f.write_char(' ')?;
            }
        }
        f.write_char(')')
    }
}

impl From<Vec<lisp::Object>> for List {
    fn from(vec: Vec<lisp::Object>) -> List {
        List { vec }
    }
}

impl ops::Deref for List {
    type Target = [lisp::Object];
    fn deref(&self) -> &Self::Target {
        &self.vec[..]
    }
}
