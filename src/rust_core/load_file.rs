use crate::environment::Environment;
use crate::ifn::IFn;
use crate::repl::Repl;
use crate::type_tag::TypeTag;
use crate::value::{ToValue, Value};
use std::rc::Rc;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct LoadFileFn {
    enclosing_environment: Arc<Environment>,
}
impl LoadFileFn {
    pub fn new(enclosing_environment: Arc<Environment>) -> LoadFileFn {
        LoadFileFn {
            enclosing_environment,
        }
    }
}
impl ToValue for LoadFileFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for LoadFileFn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        if args.len() != 1 {
            Value::Condition(format!(
                "Wrong number of arguments given to function (Given: {}, Expected: 1)",
                args.len()
            ))
        } else if let Value::String(file) = &**args.get(0).unwrap() {
            match Repl::new(self.enclosing_environment.clone()).eval_file(file) {
                Some(Value::Condition(msg)) => todo!("{}", msg),
                Some(val) => val,
                _ => Value::Nil,
            }
        } else {
            Value::Condition(format!(
                "Type mismatch; Expected instance of {}, Recieved type {}",
                TypeTag::String,
                args.len()
            ))
        }
    }
}
