use crate::error_message;
use crate::ifn::IFn;
use crate::iterable::Iterable;
// use crate::persistent_list::PersistentList;
use crate::protocol::ProtocolCastable;
use crate::type_tag::TypeTag;
use crate::types::{ImList, List};
use crate::value::{ToValue, Value};
use std::rc::Rc;

// This is a tide me over rust wrapper, as map is implemented in lower level primitives
// in pure Clojure
// // That being said, I have not decided as to whether or not there is value to having both
#[derive(Debug, Clone)]
pub struct MapFn {}
impl ToValue for MapFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for MapFn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        if args.len() != 2 {
            return error_message::wrong_arg_count(2, args.len());
        }
        let ifn_val = &**args.get(0).unwrap();
        let iterable_rc_val = args.get(1).unwrap();
        let iterable = iterable_rc_val.try_as_protocol::<Iterable>();
        match (ifn_val, iterable) {
            // @TODO first arg can be any callable, not necessarily a "function"
            (Value::IFn(ifn), Some(iterable)) => {
                let mapped = iterable
                    .iter()
                    .map(|rc_val| Rc::new(ifn.invoke(vec![rc_val])))
                    .collect::<ImList>();
                Value::List(List(mapped))
            }
            (_, None) => {
                Value::Condition(format!(
                    "Type mismatch; Expected iterable type, Recieved type {}",
                    iterable_rc_val.as_ref().type_tag(),
                ))
            }
            _ => {
                Value::Condition(format!(
                    "Type mismatch; Expected instance of {}, Recieved type {}",
                    TypeTag::IFn,
                    ifn_val.type_tag(),
                ))
            }
        }
    }
}
