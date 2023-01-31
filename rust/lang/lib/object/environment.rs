use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::ast::Identifier;
use crate::object::builtins::BuiltinsFunctions;
use crate::object::objects::Object;

#[derive(Clone, Debug, PartialEq)]
pub struct Environment {
    store: HashMap<String, Object>,
    outer: Option<Rc<RefCell<Environment>>>,
}


impl Environment {
    /// create new env
    pub fn new() -> Self {
        let mut hashmap = HashMap::new();
        Self::fill_env_with_builtins(&mut hashmap);
        Environment {
            store: hashmap,
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

    pub fn new_with_outer(outer: Rc<RefCell<Environment>>) -> Self {
        let mut hashmap = HashMap::new();
        Self::fill_env_with_builtins(&mut hashmap);
        Environment {
            store: hashmap,
            outer: Some(outer),
        }
    }
    fn fill_env_with_builtins(hashmap: &mut HashMap<String, Object>) {
        let builtins_functions = BuiltinsFunctions::new();
        let builtins = builtins_functions.get_builtins();
        for (Identifier(name), object) in builtins {
            hashmap.insert(name, object);
        }
    }
}
