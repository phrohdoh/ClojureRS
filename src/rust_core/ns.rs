use crate::environment::Environment;
use crate::ifn::IFn;
use crate::type_tag::TypeTag;
use crate::value::{ToValue, Value};
use std::rc::Rc;

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
            return Value::Condition(format!(
                "Wrong number of arguments (Given: {}, Expected: 1-2)",
                args.len()
            ));
        }

        let namespace = args.get(0).unwrap();
        match &**namespace {
            Value::Symbol(sym) => self.enclosing_environment.change_or_create_namespace(sym),
            _ => return error_message::type_mismatch(TypeTag::Symbol, &**namespace),
        }

        use crate::persistent_list::PersistentList;
        use crate::persistent_vector::PersistentVector;
        use crate::symbol::Symbol;
        use crate::keyword::Keyword;

        fn work_around_recursion_limit(ns: &Value, sym1: &Symbol, kw: &Keyword, sym2: &Symbol) {
            todo!(
                "(ns {} (:require [{} {} {}]))",
                ns,
                sym1,
                kw,
                sym2,
            ); // => 'not yet implemented: (ns my.ns (:require [clojure.string :as str]))' 💪
        }

        if_chain::if_chain! {
            // (ns my.ns (:require [clojure.string :as str]))
            //           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ 
            if let Some(rc_val) = args.get(1);
            if let Value::PersistentList(list) = &**rc_val;
            if let PersistentList::Cons(first, rest, _) = list;
            //
            // (ns my.ns (:require [clojure.string :as str]))
            //            ^^^^^^^^
            if let Value::Keyword(kw) = &**first;
            if kw.ns().is_empty();
            if kw.name() == "require";
            //
            // (ns my.ns (:require [clojure.string :as str]))
            //                     ^^^^^^^^^^^^^^^^^^^^^^^^
            if let PersistentList::Cons(p_vec, _, _) = &**rest;
            if let Value::PersistentVector(PersistentVector { vals }) = &**p_vec;
            //
            // (ns my.ns (:require [clojure.string :as str]))
            //                      ^^^^^^^^^^^^^^ ^^^ ^^^ 
            // @TODO flatten vec, get first, partition(2) rest
            if let (Some(v1), Some(v2), Some(v3)) = (vals.get(0), vals.get(1), vals.get(2));
            if let (Value::Symbol(sym1), Value::Keyword(kw), Value::Symbol(sym2)) = (&**v1, &**v2, &**v3);
            if kw.ns().is_empty();
            if kw.name() == "as";
            then {
                work_around_recursion_limit(&**namespace, sym1, kw, sym2);
            }
        }

        Value::Nil
    }
}

mod tests {
    use crate::environment;

    use {
        super::*,
        crate::{
            keyword::Keyword,
            persistent_vector::PersistentVector,
        },
    };

    #[test]
    fn with_require() {
        let env = Environment::new_main_environment();
        let env = Rc::new(env);

        // (ns my.ns (:require [clojure.string :as str]))
        let ns_macro = NsMacro::new(env.clone());
        let sym_ns = sym_rc_val!("my.ns");
        let list_require = list_rc_val!(
            keyword_val!("require")
            pvec_val!(
                sym_val!("clojure.string")
                keyword_val!("as")
                sym_val!("str")
            )
        );

        let ns_macro_args = vec![
            sym_ns,
            list_require,
        ];

        ns_macro.invoke(ns_macro_args);

        // @TODO verify `str/foo` resolves to `clojure.string/foo`
        assert_eq!(true, false);
    }
}
