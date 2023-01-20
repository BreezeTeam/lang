use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::object::objects::Object;

#[derive(Clone)]
pub struct Environment {
    store: HashMap<String, Object>,
    outer: Option<Rc<RefCell<Environment>>>,
}


impl Environment {
    /// create new env
    pub fn new() -> Self {
        Environment {
            store: Default::default(),
            outer: None,
        }
    }
    /// store identify and Object
    pub fn set(&mut self, name: &str, val: Object) {
        self.store.insert(name.to_string(), val);
    }

    /// get Object with identify
    pub fn get(&self, name: &str) -> Option<Object> {
        match self.store.get(name) {
            Some(o) => Some(o.clone()),
            None => match self.outer {
                Some(ref parent_env) => parent_env.borrow().get(name),
                None => None,
            },
        }
    }
}
