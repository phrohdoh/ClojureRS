pub trait IMeta: std::fmt::Debug {
    fn meta(&self) -> crate::types::Map;
}
