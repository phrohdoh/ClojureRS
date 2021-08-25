use std::rc::Rc;
use crate::{types::Map, value::Value};

pub trait IMap {
    fn get(&self, key: &Rc<Value>) -> Rc<Value>;
    fn get_with_default(&self, key: &Rc<Value>, default: &Rc<Value>) -> Rc<Value>;
    fn assoc(&self, key: Rc<Value>, value: Rc<Value>) -> Self;
    fn contains_key(&self,key: &Rc<Value>) -> bool;
}

impl IMap for Map {
    fn get(&self, key: &Rc<Value>) -> Rc<Value> {
        match self.0.get(key) {
            Some(val) => val.clone(),
            _ => Rc::new(Value::Nil),
        }
    }
    fn get_with_default(&self, key: &Rc<Value>, default: &Rc<Value>) -> Rc<Value> {
        match self.0.get(key) {
            Some(val) => val.clone(),
            _ => default.clone(),
        }
    }
    fn assoc(&self, key: Rc<Value>, value: Rc<Value>) -> Self {
        Self(self.0.update(key, value))
    }
    fn contains_key(&self, key: &Rc<Value>) -> bool {
        self.0.contains_key(key)
    }
}
