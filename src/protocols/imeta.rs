// use crate::persistent_list_map::PersistentListMap;
use crate::define_protocol;
use crate::traits;
use crate::symbol::Symbol;
use crate::types::ImHashMap;
use crate::types::Map;
use crate::value::ToValue;
use crate::value::Value;
use std::rc::Rc;

define_protocol!(
    IMeta = Var      | // <-- where all the magic happens
            List     |
            Vector   |
            Map      |
            Symbol //|
            // IFn
);
impl traits::IMeta for IMeta {
    fn meta(&self) -> Map {
        match &*self.value {
            // Value::PersistentList(val) => {
            //     val.meta()
            // }
            // Value::PersistentVector(val) => {
            //     val.meta()
            // }
            // Value::PersistentListMap(val) => {
            //     val.meta()
            // }
            Value::List(val) => val.meta(),
            Value::Vector(val) => val.meta(),
            Value::Map(val) => val.meta(),
            Value::Symbol(val) => {
                val.meta()
            }
            Value::Var(var) => {
                var.meta()
            }
            _ => panic!("protocols::IMeta was wrapping an invalid type {} when calling meta()",self.value.type_tag())
            // Value::IFn(val) => {
            //     val.with_meta(meta)
            // }
        }
    }
}
/// Constructs base meta if none provided
/// {:line 1
///  :column 1
///  :file "NO_SOURCE_PATH"
///  :name <something>
///  :ns <namespace>}
///
pub fn base_meta(ns: &str, name: &str) -> Map {
    let mut map = ImHashMap::new();
    map.insert(Symbol::intern("line").to_rc_value(), Value::I32(1).to_rc_value());
    map.insert(Symbol::intern("column").to_rc_value(), Value::I32(1).to_rc_value());
    map.insert(Symbol::intern("file").to_rc_value(), Value::String(String::from("NO_SOURCE_PATH")).to_rc_value());
    map.insert(Symbol::intern("ns").to_rc_value(), Value::String(String::from(ns)).to_rc_value());
    map.insert(Symbol::intern("name").to_rc_value(), Value::String(String::from(name)).to_rc_value());
    Map(map)
    // persistent_list_map!(
    //     map_entry!("line", 1_i32),
    //     map_entry!("column", 1_i32),
    //     map_entry!("file", "NO_SOURCE_PATH"),
    //     map_entry!("ns", Symbol::intern(ns)),
    //     map_entry!("name", Symbol::intern(name))
    // )
}

#[cfg(test)]
mod tests {
}
