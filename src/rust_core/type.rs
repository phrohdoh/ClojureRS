use std::rc::Rc;

use crate::{error_message, ifn::IFn, value::{ToValue, Value}};

/// ```clojure
/// (type 'foo) ;; => "clojure.lang.Symbol"
/// ```
#[derive(Debug, Clone)]
pub struct TypeFn {}
impl ToValue for TypeFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for TypeFn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        if args.len() == 1 {
            Value::String(format!("{}", args[0].type_tag()))
        } else {
            error_message::wrong_arg_count(1, args.len())
        }
    }
}
