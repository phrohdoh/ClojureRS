use crate::{traits::imeta::IMeta, types::Map};
use std::fmt::Debug;

// @TODO start swapping PersistentListMap signatures for protocol::IPersistentMap or
// with_meta<I: traits::IPersistentMap>(meta: I)

pub trait IObj: IMeta + Debug {
    fn with_meta(&self, meta: Map) -> Self;
}
