use crate::ifn::IFn;
use crate::type_tag::TypeTag;
use crate::value::{ToValue, Value};
use std::rc::Rc;

use crate::error_message;
// use crate::persistent_list::PersistentList::{Cons, Empty};
// use crate::persistent_list::ToPersistentListIter;
// use crate::persistent_vector::PersistentVector;

/// (nth coll index)
///
#[derive(Debug, Clone)]
pub struct NthFn {}
impl ToValue for NthFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for NthFn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        // @TODO generalize arity exceptions, and other exceptions
        if args.len() != 2 {
            return error_message::wrong_varg_count(&[2, 3], args.len());
        }
        // @TODO change iteration to work with Value references, or even change invoke to work on Rc<..>
        //       as we do everything else; surely we don't want to clone just to read from a collection
        if let Value::I32(ind) = **args.get(1).unwrap() {
            if ind < 0 {
                return error_message::index_cannot_be_negative(ind as usize);
            }
            let ind = ind as usize;

            match &**args.get(0).unwrap() {
                Value::List(im_list) => {
                    if im_list.len() <= ind {
                        return error_message::index_out_of_bounds(ind, im_list.len());
                    }
                    im_list.get(ind).unwrap().to_value()
                },
                Value::Vector(im_vec) => {
                    if im_vec.len() <= ind {
                        return error_message::index_out_of_bounds(ind, im_vec.len());
                    }
                    im_vec.get(ind).unwrap().as_ref().clone()
                }
                _ => error_message::type_mismatch(TypeTag::ISeq, &**args.get(0).unwrap()),
            }
        } else {
            error_message::type_mismatch(TypeTag::Integer, &**args.get(1).unwrap())
        }
    }
}
