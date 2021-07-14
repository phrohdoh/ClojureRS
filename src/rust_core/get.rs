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
    use crate::{
        keyword::Keyword,
        value::{Value, ToValue as _},
    };

    #[test] // (= (get {:k "v"} :k) "v")
    fn map_get_arity_2() {
        use super::{IFn, GetFn};
        let map = persistent_list_map!(map_entry!("k", "v")).to_rc_value();
        let key = Keyword::intern("k").to_rc_value();
        let get_fn = GetFn {};
        let args = vec![map, key];
        let val = get_fn.invoke(args);
        assert_eq!(val, "v".to_value());
    }

    #[test] // (= (get {:k "v"} :x) nil)
    fn map_get_arity_2_not_found() {
        use super::{IFn, GetFn};
        let map = persistent_list_map!(map_entry!("k", "v")).to_rc_value();
        let key = Keyword::intern("x").to_rc_value();
        let get_fn = GetFn {};
        let args = vec![map, key];
        let val = get_fn.invoke(args);
        assert_eq!(val, Value::Nil);
    }

    #[test] // (= (get {:k "v"} :k "default") "v")
    fn map_get_arity_3_found() {
        use super::{IFn, GetFn};
        let map = persistent_list_map!(map_entry!("k", "v")).to_rc_value();
        let key = Keyword::intern("k").to_rc_value();
        let default = "default".to_rc_value();
        let get_fn = GetFn {};
        let args = vec![map, key, default];
        let val = get_fn.invoke(args);
        assert_eq!(val, "v".to_value());
    }

    #[test] // (= (get {:k "v"} :x "default") "default")
    fn map_get_arity_3_not_found() {
        use super::{IFn, GetFn};
        let map = persistent_list_map!(map_entry!("k", "v")).to_rc_value();
        let key = Keyword::intern("x").to_rc_value();
        let default = "default".to_rc_value();
        let get_fn = GetFn {};
        let args = vec![map, key, default.clone()];
        let val = get_fn.invoke(args);
        assert_eq!(val, *default);
    }
}