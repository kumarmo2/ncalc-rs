use std::{collections::HashMap, rc::Rc};

#[derive(Debug, Clone)]
pub(crate) struct Context {
    _inner: Rc<HashMap<String, Object>>,
}

#[derive(Debug, Clone)]
pub(crate) enum Object {
    Int(i64),
    Double(f64),
    Bool(bool),
    Str(Rc<String>),
}
