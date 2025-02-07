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
    pub fn namespace(&self) -> Option<&str> {
        self.sym.namespace()
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
impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match (self.namespace(), self.name()) {
            (Some(ns), n) => write!(f, ":{}/{}", ns, n),
            (_, n) => write!(f, ":{}", n),
        }
    }
}
