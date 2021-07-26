use crate::error_message;
use crate::ifn::IFn;
use crate::persistent_list_map::IPersistentMap;
use crate::value::{ToValue, Value};
use std::rc::Rc;

// General get fn; however,  currently just implemented
// for our one map type, PersistentListMap
#[derive(Debug, Clone)]
pub struct GetFn {}
impl ToValue for GetFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for GetFn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        if args.len() != 2 && args.len() != 3 {
            return error_message::wrong_varg_count(&[2, 3], args.len());
        }

        if let Value::PersistentListMap(pmap) = &*(args.get(0).unwrap().clone()) {
            let key = args.get(1).unwrap();
            return if let Some(not_found) = args.get(2) {
                pmap.get_with_default(key, not_found)
            } else {
                pmap.get(key)
            }.to_value();
        }
        // @TODO add error in here with erkk's new error tools

        Value::Nil
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::keyword::Keyword;

    #[test]
    fn too_few_args_0() {
        // given
        let args = vec![
            // haystack
            // needle
        ];
        // when
        let val = GetFn {}.invoke(args);
        // then
        match val {
            Value::Condition(msg) => assert_eq!(
                msg,
                "Wrong number of arguments given to function (Given: 0, Expected: [2, 3])".to_owned()
            ),
            _ => panic!("expected to error"),
        }
    }

    #[test]
    fn too_few_args_1() {
        // given
        let haystack = persistent_list_map!().to_rc_value();

        let args = vec![
            haystack
            // needle
        ];
        // when
        let val = GetFn {}.invoke(args);
        // then
        match val {
            Value::Condition(msg) => assert_eq!(
                msg,
                "Wrong number of arguments given to function (Given: 1, Expected: [2, 3])".to_owned()
            ),
            _ => panic!("expected to error"),
        }
    }

    #[test]
    fn too_many_args_4() {
        // given
        let haystack = persistent_list_map!(map_entry!("k", "v")).to_rc_value();
        let needle = Value::Keyword(Keyword::intern("k")).to_rc_value();

        let args = vec![
            haystack,
            needle,
            Rc::new(Value::Nil),
            Rc::new(Value::Nil),
        ];
        // when
        let val = GetFn {}.invoke(args);
        // then
        match val {
            Value::Condition(msg) => assert_eq!(
                msg,
                "Wrong number of arguments given to function (Given: 4, Expected: [2, 3])".to_owned()
            ),
            _ => panic!("expected to error"),
        }
    }

    mod arity_2 {
        use super::*;

        #[test]
        fn returns_associated_val_when_key_found() {
            // given
            let haystack = persistent_list_map!(map_entry!("k", "v")).to_rc_value();
            let needle = Value::Keyword(Keyword::intern("k")).to_rc_value();

            let args = vec![
                haystack,
                needle.clone(),
            ];
            // when
            let val = GetFn {}.invoke(args);
            // then
            assert_eq!(val, Value::String("v".to_string()));
        }

        #[test]
        fn returns_nil_when_key_not_found() {
            // given
            let haystack = persistent_list_map!(map_entry!("x", "v")).to_rc_value();
            let needle = Value::Keyword(Keyword::intern("k")).to_rc_value();

            let args = vec![
                haystack,
                needle.clone(),
            ];
            // when
            let val = GetFn {}.invoke(args);
            // then
            assert_eq!(val, Value::Nil);
        }

        #[test]
        fn returns_nil_when_key_not_found_in_empty() {
            // given
            let haystack = persistent_list_map!(/* empty */).to_rc_value();
            let needle = Value::Keyword(Keyword::intern("k")).to_rc_value();

            let args = vec![
                haystack,
                needle.clone(),
            ];
            // when
            let val = GetFn {}.invoke(args);
            // then
            assert_eq!(val, Value::Nil);
        }
    }

    mod arity_3 {
        use super::*;

        #[test]
        fn returns_associated_val_when_key_found() {
            // given
            let haystack = persistent_list_map!(map_entry!("k", "v")).to_rc_value();
            let needle = Value::Keyword(Keyword::intern("k")).to_rc_value();

            let args = vec![
                haystack,
                needle.clone(),
                Rc::new(Value::Nil),
            ];
            // when
            let val = GetFn {}.invoke(args);
            // then
            assert_eq!(val, Value::String("v".to_string()));
        }

        #[test]
        fn returns_not_found_val_when_key_not_found() {
            // given
            let haystack = persistent_list_map!(map_entry!("x", "v")).to_rc_value();
            let needle = Value::Keyword(Keyword::intern("k")).to_rc_value();
            let if_needle_not_found = Value::Keyword(Keyword::intern("not-found")).to_rc_value();

            let args = vec![
                haystack,
                needle,
                if_needle_not_found.clone(),
            ];
            // when
            let val = GetFn {}.invoke(args);
            // then
            assert_eq!(&val, if_needle_not_found.as_ref());
        }

        #[test]
        fn returns_not_found_val_when_key_not_found_in_empty() {
            // given
            let haystack = persistent_list_map!(/* empty */).to_rc_value();
            let needle = Value::Keyword(Keyword::intern("k")).to_rc_value();
            let if_needle_not_found = Value::Keyword(Keyword::intern("not-found")).to_rc_value();

            let args = vec![
                haystack,
                needle,
                if_needle_not_found.clone(),
            ];
            // when
            let val = GetFn {}.invoke(args);
            // then
            assert_eq!(&val, if_needle_not_found.as_ref());
        }
    }
}
