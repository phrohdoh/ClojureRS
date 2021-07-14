use crate::symbol::Symbol;
use std::fmt;
use std::hash::Hash;

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Keyword {
    // In Clojure proper,  a Keyword wraps a Symbol to share their ..symbolic functionality
    pub sym: Symbol,
}
impl Keyword {
    pub fn name(&self) -> &str {
        self.sym.name()
    }
    pub fn ns(&self) -> &str {
        &self.sym.ns
    }
    pub fn intern(name: &str) -> Keyword {
        Keyword {
            sym: Symbol::intern(name),
        }
    }
    // Note; normally 'with_x' would imply x is the second argument
    // here, but we are keeping the semantics of interning that
    // Clojure proper has
    pub fn intern_with_ns(ns: &str, name: &str) -> Keyword {
        Keyword {
            sym: Symbol::intern_with_ns(ns, name),
        }
    }
}
macro_rules! keyword {
    ($n:expr) => {crate::keyword::Keyword::intern($n)};
    ($ns:expr, $n:expr) => {crate::keyword::Keyword::intern_with_ns($ns,$n)};
}
macro_rules! keyword_val {
    ($n:expr) => {keyword!($n).to_value()};
    ($ns:expr,$n:expr) => {keyword!($ns,$n).to_value()};
}
macro_rules! keyword_rc_val {
    ($n:expr) => {keyword_val!($n).to_rc_value()};
    ($ns:expr,$n:expr) => {keyword_val!($ns,$n).to_rc_value()};
}
impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.sym.ns != "" {
            write!(f, ":{}/{}", self.sym.ns, self.sym.name)
        } else {
            write!(f, ":{}", self.sym.name)
        }
    }
}
