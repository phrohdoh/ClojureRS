use std::rc::Rc;
use crate::environment::Environment;
use crate::ifn::IFn;
use crate::persistent_list::PersistentList;
use crate::persistent_vector::PersistentVector;
use crate::type_tag::TypeTag;
use crate::value::{ToValue, Value};

use crate::error_message;

#[derive(Debug, Clone)]
pub struct NsMacro {
    enclosing_environment: Rc<Environment>,
}
impl NsMacro {
    pub fn new(enclosing_environment: Rc<Environment>) -> NsMacro {
        NsMacro {
            enclosing_environment,
        }
    }
}
impl ToValue for NsMacro {
    fn to_value(&self) -> Value {
        Value::Macro(Rc::new(self.clone()))
    }
}
impl IFn for NsMacro {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        if args.len() != 1 && args.len() != 2 {
            return error_message::wrong_varg_count(
                &[1, 2],
                args.len(),
            );
        }

        let ns_val = args.get(0).unwrap();
        match &**ns_val {
            Value::Symbol(ns_sym) => {
                self.enclosing_environment.change_or_create_namespace(ns_sym);
            }
            _ => return error_message::type_mismatch(TypeTag::Symbol, &**ns_val),
        }

        // fn list_headed_with_keyword_require(list_val: &Value) -> Option<(&Rc<Value>, &Rc<PersistentList>)> {
        fn list_headed_with_keyword_require(list_val: &Value) -> Option<&Rc<PersistentList>> {
            if_chain::if_chain! {
                if let Value::PersistentList(list) = list_val;
                if let PersistentList::Cons(head_kw, tail, _) = list;
                if let Value::Keyword(kw) = &**head_kw;
                if kw.namespace().is_none();
                if kw.name() == "require";
                // then { Some((head, tail)) }
                then { Some(tail) }
                else { None }
            }
        }

        let mut end_iter = false;
        // if let Some((x,y)) = args.iter()
        if let Some(y) = args.iter()
            .skip(1)
            .filter_map(|x| {
                if end_iter {
                    None
                } else {
                    let ret = list_headed_with_keyword_require(x.as_ref());
                    if ret.is_none() {
                        end_iter = true;
                    }
                    ret
                }
            })
            .nth(0) {
                if_chain::if_chain! {
                    if let PersistentList::Cons(head_vec, _tail, _) = &**y;
                    if let Value::PersistentVector(PersistentVector { vals }) = &**head_vec;
                    if !vals.is_empty(); // @TODO error otherwise
                    let ns_sym_rc_val = &vals[0];
                    let ns_sym_val = &**ns_sym_rc_val;
                    if let Value::Symbol(ns_sym) = ns_sym_val; // @TODO error otherwise
                    then {
                        // 🤔
                        self.enclosing_environment.add_referred_namespace_to_curr_namespace(ns_sym);
                    }
                }
            }

        Value::Nil
    }
}

#[cfg(test)]
mod tests {
    use crate::{keyword::Keyword, persistent_vector::PersistentVector, symbol::Symbol};
    use super::*;

    /// ```clojure
    /// (ns my.ns (:require [some-ns]))
    /// ```
    #[test]
    fn ns_with_require() {
        // given
        let env = Environment::clojure_core_environment();
        assert!(env.as_ref().get_current_namespace_name() != "my.ns".to_owned());
        let ns_macro = NsMacro { enclosing_environment: env.clone() };
        let args = vec![
            Symbol::intern("my.ns").to_rc_value(),
            list_val!(
                Keyword::intern("require")
                PersistentVector {
                    vals: vec![Symbol::intern("some-ns").to_rc_value()],
                }
            ).to_rc_value(),
        ];
        // when
        assert!(env.as_ref().get_current_namespace_name() != "my.ns".to_owned());
        let _ = ns_macro.invoke(args);
        // then
        assert_eq!(env.as_ref().get_current_namespace_name(), "my.ns".to_owned());
        assert!(env.has_namespace(&Symbol::intern("some-ns")));
    }
}
