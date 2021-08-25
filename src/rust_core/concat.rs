use crate::ifn::IFn;
use crate::types::List;
use crate::value::{ToValue, Value};
use std::rc::Rc;

/// (concat x y & zs)
///
#[derive(Debug, Clone)]
pub struct ConcatFn {}
impl ToValue for ConcatFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for ConcatFn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        let concatted_vec = args.iter().fold(Vec::new(), |mut sum, coll| {
            let coll_vec = match &**coll {
                Value::List(list) => {
                    list.iter().map(Clone::clone).collect()
                }
                Value::Vector(vector) => {
                    vector.iter().map(Clone::clone).collect()
                }
                _ => vec![],
            };

            sum.extend(coll_vec);
            sum
        });
        Value::List(List(concatted_vec.into()))
    }
}
