//! General map utilities
use crate::{types::{Map, ImHashMap}, value::Value};
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MapEntry {
    // We need to box key to avoid an infinite cycle of
    // Value::*Map { *Map { MapEntry { Value <--- cycle restarts , val }}}
    // Implemented with an Rc because inevitably,  our system tends to live in Rcs
    pub key: Rc<Value>,
    pub val: Rc<Value>,
}

/// ```rust,no_run
/// # #[macro_use] extern crate rust_clojure; use rust_clojure::*;
/// map_entry!("doc", "this is a docstring");
/// ```
///
/// equals
///
/// ```clojure
/// {:doc "this is a docstring"}
/// ```
#[macro_export]
macro_rules! map_entry {
    ($key:expr, $value:expr) => {{
        $crate::maps::MapEntry {
            key: $crate::value::ToValue::to_rc_value(&$crate::keyword::Keyword::intern($key)),
            val: $crate::value::ToValue::to_rc_value($value),
        }
    }};
}

impl From<(Rc<Value>, Rc<Value>)> for MapEntry {
    fn from((k, v): (Rc<Value>, Rc<Value>)) -> Self {
        Self {
            key: k,
            val: v,
        }
    }
}

impl From<(&Rc<Value>, &Rc<Value>)> for MapEntry {
    fn from((k, v): (&Rc<Value>, &Rc<Value>)) -> Self {
        Self {
            key: k.clone(),
            val: v.clone(),
        }
    }
}

impl From<Vec<MapEntry>> for Map {
    fn from(entries: Vec<MapEntry>) -> Self {
        let map = entries.into_iter()
            .map(|entry| (entry.key, entry.val))
            .collect::<ImHashMap>();
        Self(map)
    }
}

impl From<MapEntry> for (Rc<Value>, Rc<Value>) {
    fn from(entry: MapEntry) -> Self {
        (entry.key, entry.val)
    }
}
