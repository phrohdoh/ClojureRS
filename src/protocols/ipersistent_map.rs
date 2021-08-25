use crate::value::Value;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct IPersistentMap {
    value: Rc<Value>,
}
impl crate::protocol::Protocol for IPersistentMap {
    fn raw_wrap(val: &Rc<Value>) -> Self {
        Self { value: val.clone() }
    }
    fn raw_unwrap(&self) -> Rc<Value> {
        self.value.clone()
    }
    fn instanceof(val: &Rc<Value>) -> bool {
        matches!(val.as_ref(), Value::Map(_))
    }
}
