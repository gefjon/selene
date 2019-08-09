use std::{boxed::Box, collections::HashSet, cmp, fmt, sync::{RwLock, PoisonError}};
use lazy_static::lazy_static;

lazy_static! {
    static ref SYMBOLS: RwLock<HashSet<&'static str>> = RwLock::new(HashSet::new());
}

#[derive(Copy, Clone, Eq)]
pub struct Symbol {
    name: &'static str,
}

impl cmp::PartialEq<Symbol> for Symbol {
    fn eq(&self, other: &Symbol) -> bool {
        (self.name as *const str) == (other.name as *const str)
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.name)
    }
}

fn finalize_symbol(s: &'_ str) -> &'static str {
    Box::leak(s.to_owned().into())
}

fn try_get_sym(s: &'_ str) -> Option<&'static str> {
    SYMBOLS.read().unwrap_or_else(PoisonError::into_inner).get(s).cloned()
}

/// must have already checked that `s` is not in `SYMBOLS`, because this allocs every time
fn insert_sym(s: &'_ str) -> &'static str {
    let s = finalize_symbol(s);
    SYMBOLS.write().unwrap_or_else(PoisonError::into_inner).insert(s);
    s
}

impl Symbol {
    pub fn intern(s: &str) -> Symbol {
        let name: &'static str = try_get_sym(s).unwrap_or_else(|| insert_sym(s));
        Symbol { name }
    }
}
