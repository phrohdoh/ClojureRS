use crate::types::ImHashMapIter;
use crate::types::ImListIter;
use crate::types::ImVectorIter;
use crate::types::Vector;
use crate::value::Value;
use std::rc::Rc;
// @TODO move to protocols::iterable

#[derive(Debug, Clone)]
pub struct Iterable {
    value: Rc<Value>
}
impl crate::protocol::Protocol for Iterable {
    fn raw_wrap(val: &Rc<Value>) -> Self {
        Self { value: Rc::clone(val) }
    }
    fn raw_unwrap(&self) -> Rc<Value> {
        Rc::clone(&self.value)
    }
    fn instanceof(val: &Rc<Value>) -> bool {
        matches!(val.as_ref(), Value::List(_))
    }
}

pub enum IterableIter<'source> {
    List(ImListIter<'source>),
    Vector(ImVectorIter<'source>),
    HashMap(ImHashMapIter<'source>),
}
impl<'i> Iterator for IterableIter<'i> {
    type Item = Rc<Value>;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            IterableIter::List(iter) => iter.next().map(Clone::clone),
            IterableIter::Vector(iter) => iter.next().map(Clone::clone),
            IterableIter::HashMap(iter) => {
                let maybe_map_entry = iter.next();
                if let Some((key, val)) = maybe_map_entry {
                    // In Clojure: [key val]
                    return Some(Rc::new(Value::Vector(
                        Vector(vec![key.clone(), val.clone()].into())
                    )));
                }
                None
            }
        }
    }
}
impl Iterable {
    pub fn iter(&self) -> IterableIter {
        match self.value.as_ref() {
            Value::List(list) => IterableIter::List(list.iter()),
            Value::Vector(vector) => IterableIter::Vector(vector.iter()),
            Value::Map(map) => IterableIter::HashMap(map.iter()),
            _ => panic!("Called Iterable iter on non-iterable"),
        }
    }
}
