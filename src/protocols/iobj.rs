use crate::protocol::ProtocolCastable;
use crate::define_protocol;
use crate::types::Map;
use crate::value::{Value,ToValue};
use crate::traits;
use std::rc::Rc;
// TODO allow nullable protocols 
define_protocol!(
    IObj = List     |
           Vector   |
           Map      |
           Symbol //|
           // IFn
);
impl traits::IMeta for IObj {
    fn meta(&self) -> Map {
        match &*self.value {
            Value::List(val) => val.meta(),
            Value::Vector(val) => val.meta(),
            Value::Map(val) => val.meta(),
            Value::Symbol(val) => val.meta(),
            _ => {
                panic!("protocols::IMeta was wrapping an invalid type {} when calling meta()",self.value.type_tag())
                //Map::empty()
            }
            // Value::IFn(val) => {
            //     val.with_meta(meta)
            // }
        }
    }
}
impl traits::IObj for IObj {
    fn with_meta(&self,meta: Map) -> IObj {
        match &*self.value {
            Value::List(val) => {
                val.with_meta(meta).to_rc_value().as_protocol::<IObj>()
            }
            Value::Vector(val) => {
                val.with_meta(meta).to_rc_value().as_protocol::<IObj>()
            }
            Value::Map(val) => {
                val.with_meta(meta).to_rc_value().as_protocol::<IObj>()
            }
            Value::Symbol(val) => {
                val.with_meta(meta).to_rc_value().as_protocol::<IObj>()
            }
            _ => {
                panic!("protocols::IMeta was wrapping an invalid type {} when calling meta()",self.value.type_tag())
            }
            // Value::IFn(val) => {
            //     val.with_meta(meta)
            // }
        }
    }
}

#[cfg(test)]
mod tests {
}
