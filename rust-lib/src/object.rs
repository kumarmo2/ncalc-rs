use std::{collections::HashMap, rc::Rc};

#[derive(Debug, Clone, Default)]
pub struct Context {
    _inner: Rc<HashMap<String, Object>>,
}

impl Context {
    pub(crate) fn get(&self, key: &str) -> Option<Object> {
        self._inner.get(key).map(|obj| obj.clone())
    }

    pub(crate) fn from_map(map: HashMap<String, Object>) -> Self {
        Self {
            _inner: Rc::new(map),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Int(i64),
    Double(f64),
    Bool(bool),
    Str(Rc<String>),
}
