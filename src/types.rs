use std::{fmt, ops::Deref, rc::Rc, sync::Arc};
use crate::{environment::Environment, maps::MapEntry, traits, value::{Evaluable, ToValue, Value}};

type CljT = Rc<Value>;
type CljK = Rc<Value>;
type CljV = Rc<Value>;
pub(crate) type ImList    = im_rc::Vector<CljT>;
pub(crate) type ImVector  = im_rc::Vector<CljT>;
pub(crate) type ImHashMap = im_rc::HashMap<CljK, CljV>;
pub(crate) type ImListIter<'i>    = im_rc::vector::Iter<'i, CljT>;
pub(crate) type ImVectorIter<'i>  = im_rc::vector::Iter<'i, CljT>;
pub(crate) type ImHashMapIter<'i> = im_rc::hashmap::Iter<'i, CljK, CljV>;

#[derive(Eq, Debug, Clone, PartialEq, Hash)]
pub struct List(pub(crate) ImList);
impl List {
    pub fn empty() -> Self {
        Self(ImList::new())
    }
    /// construct a list containing a single value
    pub fn unit<V: ToValue>(v: V) -> Self {
        Self(ImList::unit(v.to_rc_value()))
    }
    /// prepend a value `v` to, and return, a new list based off of `self`
    pub(crate) fn conj<V: ToValue>(&self, v: V) -> Self {
        let v = v.to_rc_value();
        let mut slf = self.0.clone();
        slf.insert(0, v);
        Self(slf)
    }
}
impl Deref for List {
    type Target = ImList;
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl From<ImList> for List {
    fn from(l: ImList) -> Self { Self(l) }
}
impl Evaluable for List {
    fn eval_to_rc(&self, environment: Arc<Environment>) -> Rc<Value> {
        self.to_rc_value().eval_to_rc(environment)
    }
}

/// ```rust
/// list!(sym!("+") 1 2); // (+ 1 2) as List
/// ```
/// Meant to look closer to Clojure's native list syntax, to give us some Clojuresque sugar 
#[macro_export]
macro_rules! list {
    ($($val:expr)*) => {{
        use std::convert::Into as _;
        let mut rc_vals = vec![];
        $(
            rc_vals.push($crate::value::ToValue::to_rc_value(&$val));
        )*
        let inner = rc_vals.into();
        let list = $crate::types::List(inner);
        list
    }};
}

/// ```rust
/// list_val!(sym!("+") 1 2); // (+ 1 2) as Value (Value::List)
/// ```
/// Meant to look closer to Clojure's native list syntax, to give us some Clojuresque sugar 
#[macro_export]
macro_rules! list_val {
    ($($val:expr)*) => {{
        use $crate::value::ToValue;
        use std::convert::Into as _;
        let mut rc_vals = vec![];
        $(
            rc_vals.push(ToValue::to_rc_value(&$val));
        )*
        let inner = rc_vals.into();
        let list = $crate::types::List(inner);
        let val = ToValue::to_value(&list);
        val
    }};
}

/// ```rust
/// list_rc_val!(sym!("+") 1 2); // (+ 1 2) as Rc<Value> (Value::List)
/// ```
/// Meant to look closer to Clojure's native list syntax, to give us some Clojuresque sugar 
#[macro_export]
macro_rules! list_rc_val {
    ($($val:expr)*) => {{
        use $crate::value::ToValue;
        use std::convert::Into as _;
        let mut rc_vals = vec![];
        $(
            rc_vals.push(ToValue::to_rc_value(&$val));
        )*
        let inner = rc_vals.into();
        let list = $crate::types::List(inner);
        let rc_val = ToValue::to_rc_value(&list);
        rc_val
    }};
}

#[derive(Eq, Debug, Clone, PartialEq, Hash)]
pub struct Vector(pub(crate) ImVector);
impl Vector {
    pub fn empty() -> Self {
        Self(ImVector::new())
    }
    /// append a value `v` to, and return, a new vector based off of `self`
    pub(crate) fn conj<V: ToValue>(&self, v: V) -> Self {
        let v = v.to_rc_value();
        let mut slf = self.0.clone();
        slf.insert(slf.len(), v);
        Self(slf)
    }
}
impl Deref for Vector {
    type Target = ImVector;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Eq, Debug, Clone, PartialEq, Hash)]
pub struct Map(pub(crate) ImHashMap);
impl Map {
    pub fn empty() -> Self {
        Self(ImHashMap::new())
    }
    /// update an entry into, and return, a new map based off of `self`
    pub(crate) fn conj<E: Into<MapEntry>>(&self, entry: E) -> Self {
        let MapEntry { key, val } = entry.into();
        let slf = self.0.update(key, val);
        Self(slf)
    }
}
impl Deref for Map {
    type Target = ImHashMap;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[macro_export]
macro_rules! map {
    () => {{
        $crate::types::Map(vec![].into())
    }};
    ($entry:expr) => {{
        let $crate::maps::MapEntry { key, val } = $entry.into();
        let key = $crate::value::ToValue::to_rc_value(&key);
        let val = $crate::value::ToValue::to_rc_value(&val);
        $crate::types::Map(vec![(key, val)].into())
    }};
}

impl traits::IMeta for List {
    fn meta(&self) -> Map {
        // @TODO implement
        Map::empty()
    }
}
impl traits::IMeta for Vector {
    fn meta(&self) -> Map {
        // @TODO implement
        Map::empty()
    }
}
impl traits::IMeta for Map {
    fn meta(&self) -> Map {
        // @TODO implement
        Map::empty()
    }
}

impl traits::IObj for List {
    fn with_meta(&self, meta: Map) -> Self {
        // @TODO implement
        self.clone()
    }
}
impl traits::IObj for Vector {
    fn with_meta(&self, meta: Map) -> Self {
        // @TODO implement
        self.clone()
    }
}
impl traits::IObj for Map {
    fn with_meta(&self, meta: Map) -> Self {
        // @TODO implement
        self.clone()
    }
}

#[macro_export]
macro_rules! conj {
    ($map:expr, $($kv:expr),*) => {{
        let mut entries = $map.iter()
            .map(::std::convert::Into::<_>::into)
            .collect::<::std::vec::Vec<$crate::maps::MapEntry>>();
        $({
            let kv_entry = ::std::convert::Into::<$crate::maps::MapEntry>::into($kv);
            entries.push(kv_entry);
        })*
        ::std::convert::Into::<_>::into(entries)
    }};
}

#[macro_export]
macro_rules! merge {
    ($m1:expr,$($mN:expr),*) => {{
        let mut entries = $m1.iter()
            .map(::std::convert::Into::<_>::into)
            .collect::<::std::vec::Vec<$crate::maps::MapEntry>>();
        $({
            let mN_entries = $mN.iter()
                .map(::std::convert::Into::<_>::into)
                .collect::<::std::vec::Vec<$crate::maps::MapEntry>>();
            entries.extend_from_slice(&mN_entries);
        })*
        ::std::convert::Into::<_>::into(entries)
    }};
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "()")
        } else {
            let str = self.iter()
                .map(|rc_val| rc_val.as_ref().to_string_explicit())
                .collect::<Vec<_>>()
                .join(" ");
            write!(f, "({})", str)
        }
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "[]")
        } else {
            let str = self.iter()
                .map(|rc_val| rc_val.as_ref().to_string_explicit())
                .collect::<Vec<_>>()
                .join(" ");
            write!(f, "[{}]", str)
        }
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "[]")
        } else {
            let str = self.iter()
                .map(|(k, v)| format!("{} {}", k.to_string_explicit(), v.to_string_explicit())) 
                .collect::<Vec<_>>()
                .join(", ");
            write!(f, "{{{}}}", str)
        }
    }
}
