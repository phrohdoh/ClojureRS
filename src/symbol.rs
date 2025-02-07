use crate::persistent_list_map::PersistentListMap;
use crate::traits;
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Eq, Clone, Debug)]
pub struct Symbol {
    pub name: String,
    pub ns: Option<String>,
    pub meta: PersistentListMap,
}
macro_rules! sym {
    ($x:expr) => {
        Symbol::intern($x)
    }
}
impl Hash for Symbol {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (&self.name,self.ns.as_ref()).hash(state);
    }
}
impl Symbol {
    pub fn intern(name: &str) -> Symbol {
        let mut ns = None;
        let mut name = name;
        // @TODO See if we will have any problems with manipulating
        //       text in other languages
        //       I think we will be ok here though
        if let Some(ind) = name.chars().position(|c| c == '/') {
            // @TODO Make sure that the index given by ^
            //       has the same meaning as the index
            //       we are giving to this range
            //       Ie,  if the 6 in a[..6] refers to the 6th byte
            //       and  if the 6 in Some(6) means the 6th character,
            //       we need to make sure each 'character' in this case
            //       is 1 byte, and not some other grapheme abstraction
            //       else,these are two different indexes

            // support interning of the symbol '/' for division
            if ind > 0 || name.len() > 1 {
                let ns_name = &name[..ind];
                if !ns_name.is_empty() {
                    ns = Some(ns_name);
                }
                name = &name[ind + 1..];
            }
        }
        match ns {
            Some(ns) => Symbol::intern_with_ns(ns, name),
            _ => Symbol::intern_(name),
        }
    }
    pub fn intern_with_ns(ns: &str, name: &str) -> Symbol {
        // assert!(ns.len() > 0);
        Symbol {
            name: String::from(name),
            ns: if ns.is_empty() { None } else { String::from(ns).into() },
            meta: PersistentListMap::Empty,
        }
    }
    pub fn unqualified(&self) -> Symbol {
        // So we can keep the same meta 
        let mut retval = self.clone();
        retval.ns = None;
        retval
    }
    pub fn has_ns(&self) -> bool {
        self.namespace().is_some()
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn namespace(&self) -> Option<&str> {
        self.ns.as_ref().map(|x| x.as_str())
    }
    // @TODO use IPersistentMap instead perhaps 
    pub fn meta(&self) -> PersistentListMap {
        self.meta.clone()
    }
    pub fn with_meta(&self, meta: PersistentListMap) -> Symbol {
        Symbol {
            name: self.name.clone(), // String::from(self.name.clone()),
            ns: self.ns.clone(),        // String::from(self.ns.clone()),
            meta,
        }
    }
    fn intern_(name: &str) -> Self {
        Self {
            name: String::from(name),
            ns: None,
            meta: PersistentListMap::Empty,
        }
    }
}
impl PartialEq for Symbol {
    // Remember; meta doesn't factor into equality
    fn eq(&self,other: &Self) -> bool {
        self.name == other.name && self.ns == other.ns 
    }
}
impl traits::IMeta for Symbol {
    fn meta(&self) -> PersistentListMap {
        self.meta()
    }
}
impl traits::IObj for Symbol {
    fn with_meta(&self,meta: PersistentListMap) -> Symbol {
        self.with_meta(meta)
    }
}
impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.ns {
            Some(ns) => write!(f, "{}/{}", ns, self.name),
            _ => write!(f, "{}", self.name),
        }
    }
}

#[cfg(test)]
mod tests {

    mod symbol_tests {
        use crate::maps::MapEntry;
        use crate::persistent_list_map::ToPersistentListMapIter;
        use crate::persistent_list_map::PersistentListMap;
        use crate::symbol::Symbol;
        use std::collections::HashMap;

        #[test]
        fn test_intern() {
            assert_eq!(
                Symbol::intern("a"),
                Symbol {
                    ns: None,
                    name: String::from("a"),
                    meta: PersistentListMap::Empty,
                }
            );
        }

        #[test]
        fn test_intern_with_ns() {
            assert_eq!(
                Symbol::intern_with_ns("clojure.core", "a"),
                Symbol {
                    ns: String::from("clojure.core").into(),
                    name: String::from("a"),
                    meta: PersistentListMap::Empty
                }
            );
            assert_eq!(
                Symbol::intern("a"),
                Symbol {
                    ns: None,
                    name: String::from("a"),
                    meta: PersistentListMap::Empty
                }
            );
            assert_eq!(
                Symbol::intern("clojure.core/a"),
                Symbol {
                    ns: String::from("clojure.core").into(),
                    name: String::from("a"),
                    meta: PersistentListMap::Empty
                }
            );
            assert_eq!(
                Symbol::intern("clojure/a"),
                Symbol {
                    ns: String::from("clojure").into(),
                    name: String::from("a"),
                    meta: PersistentListMap::Empty,
                }
            );
            assert_eq!(
                Symbol::intern("/a"),
                Symbol {
                    ns: None,
                    name: String::from("a"),
                    meta: PersistentListMap::Empty,
                }
            );
        }
        #[test]
        fn test_with_meta() {
            assert_eq!(
                Symbol::intern_with_ns(
                    "namespace",
                    "name"
                ).with_meta(
                    persistent_list_map!(map_entry!("key", "value"))
                ),
                Symbol {
                    ns: String::from("namespace").into(),
                    name: String::from("name"),
                    meta: persistent_list_map!(map_entry!("key", "value"))
                }
            );
            assert_eq!(
                Symbol::intern_with_ns(
                    "namespace",
                    "name"
                ).with_meta(
                    conj!(
                        PersistentListMap::Empty,
                        map_entry!("key", "value")
                    )
                ),
                Symbol {
                    ns: String::from("namespace").into(),
                    name: String::from("name"),
                    meta: conj!(
                        PersistentListMap::Empty,
                        map_entry!("key", "value")
                    )
                }
            );
        }

        #[test]
        fn test_work_with_hashmap() {
            let mut hashmap = HashMap::new();
            hashmap.insert(Symbol::intern("+"), 1_i32);
            hashmap.insert(Symbol::intern("-"), 2_i32);

            assert_eq!(1_i32, *hashmap.get(&Symbol::intern("+")).unwrap());
            assert_eq!(2_i32, *hashmap.get(&Symbol::intern("-")).unwrap());
            assert_eq!(None, hashmap.get(&Symbol::intern("*")));
        }
    }
}
