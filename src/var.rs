use crate::symbol::Symbol;
// use crate::persistent_list_map::PersistentListMap;
use crate::types::Map;
use crate::value::{ToValue, Value};
use crate::traits;
use crate::ifn::IFn;
use std::fmt;
use std::hash::{Hash,Hasher};
use crate::protocols;
use crate::iterable::Iterable;
use std::rc::Rc;
use std::cell::RefCell;
use crate::protocol::Protocol;
use crate::protocol::ProtocolCastable;

#[derive(Clone, Debug)]
pub struct Var {
    // Normally in Clojure, this references an actual Namespace. However, hard
    // references are more expensive, logically, in Rust, not to mention it
    // makes this harder to reason about -- its much easier to deal with data
    // (even partially mutable data), rather than something intertwined with the
    // living system in some way by referencing and interacting with another
    // piece
    pub ns : Symbol,
    pub sym: Symbol,
    // Another decision we will have to make; meta can be any IPersistentMap,
    // but as we know, rust is not so friendly for passing around trait objects,
    // and so we avoid them. For now, we will use a PersistentListMap, but I
    // believe this is actually a perfect time to use our Protocols, since our Protocols
    // allow us to we
    // don't need to be extending any Rust values with these protocols [see note on Protocols
    // in design document, once they arrive] (and if
    // we do, perhaps this is time to think on how we're going to represent Rust
    // types generically with Values anyways)
    //
    // Other note; all values here except the meta and `root` should be
    // immutable, is there value in expressing this mixed mutability in someway
    // without just wrapping these in RefCells?
    meta: RefCell<protocols::IPersistentMap>,
    pub root: RefCell<Rc<Value>>,
}
macro_rules! var{
    ($ns:expr, $sym:expr) => {
        Var::intern(sym!($ns),sym!($sym))
    }
}
impl Var {

    // Note; not quite the same as Clojure's intern, because this does not directly reference the living
    // Its possible we should call this create or something instead, and basically not use intern at all
    pub fn intern(ns: Symbol,sym: Symbol) -> Var {
        let empty_meta = Map::empty().to_rc_value();
        Var {
            ns,
            sym,
            meta: RefCell::new(empty_meta.as_protocol::<protocols::IPersistentMap>()),
            // What do if unbound? Why does unbound exist?
            root: RefCell::new(Value::Nil.to_rc_value())
        }
    }

    pub fn deref(&self) -> Rc<Value> {
        self.root.borrow().clone() 
    }

    pub fn bind_root(&self, root: Rc<Value>){
        self.root.replace(root);
    }

    pub fn set_meta(&self, meta: Map) {
        self.meta.replace_with(|_| meta.to_rc_value().as_protocol::<protocols::IPersistentMap>() );
    }
    // @TODO swap out Iterable for ISeq
    // Also, this cannot return a Condition until we decide how we want to represent Conditions
    // in a function that returns a Protocol'ed value -- let's handle that next, as we add Conditions
    pub fn alter_meta(&self,alter: protocols::IFn,args: Iterable) -> protocols::IPersistentMap {
        self.meta.replace_with(|meta| {
           // @TODO add proper prepending
            let mut new_args = vec![Rc::clone(&meta.unwrap())];
            new_args.extend_from_slice(&args.iter().collect::<Vec<Rc<Value>>>());
            
            let maybe_updated_meta = Rc::new(alter.invoke(new_args)).try_as_protocol::<protocols::IPersistentMap>();
            if let Some(updated_meta) = maybe_updated_meta {
                updated_meta
            }
            else {
                meta.clone()
            }
        })
    }

}
impl PartialEq for Var {
    // Remember; meta doesn't factor into equality
    fn eq(&self,other: &Self) -> bool {
        self.ns == other.ns && self.sym == other.sym
    }
}
impl Hash for Var {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (&self.ns,&self.sym).hash(state);
    }
}
impl traits::IMeta for Var {
    fn meta(&self) -> crate::types::Map {
        let meta_rc_val = self.meta.borrow().unwrap();
        match &*meta_rc_val {
            Value::Map(meta_map) => {
                meta_map.clone()
            }
            _ => panic!("In var.rs, meta(); IPersistentMap failed to unwrap to Value::Map")

        }
    }
}

// impl traits::IObj for Var {
//     fn with_meta(&self,meta: PersistentListMap) -> Symbol {
//         self.with_meta(meta)
//     }
// }

impl fmt::Display for Var {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"#'{}",self.sym)
    }
}

#[cfg(test)]
mod tests {
    use crate::var::Var;
    use crate::symbol::Symbol;
    use std::rc::Rc;
    use crate::value::Value;
    
    #[test]
    fn deref(){
        let a = sym!("clojure");
        assert!(a == Symbol::intern("clojure"));

        let v = var!("clojure.core","+");
        v.bind_root(Rc::new(Value::I32(12)));

        assert!(*v.deref() == Value::I32(12));

        v.bind_root(Rc::new(Value::I32(25)));

        assert!(*v.deref() == Value::I32(25));
    }
}
