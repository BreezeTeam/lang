use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use crate::ast::{BlockStatement, Identifier};
use crate::object::environment::Environment;

pub type BuiltinFunction = fn(Vec<Object>) -> Result<Object, String>;

#[derive(Clone, Debug, PartialEq)]
pub enum Object {
    Integer(i64),
    Boolean(bool),
    String(String),
    NULL,
    // list of elements
    Array(Vec<Object>),
    Function(
        // Parameters
        Vec<Identifier>,
        // Body
        BlockStatement,
        // Function Env
        Rc<RefCell<Environment>>,
    ),
    Builtin(String, usize, BuiltinFunction),
    Hash(HashMap<Object, Object>),
    Return(Box<Object>),
    // error message
    Error(String),
}

impl Eq for Object {}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Object::Integer(ref i) => write!(f, "{}", i),
            Object::Boolean(ref b) => {
                if *b {
                    write!(f, "true")
                } else {
                    write!(f, "false")
                }
            }
            Object::String(ref s) => write!(f, "{}", s),
            Object::NULL => write!(f, "null"),
            Object::Array(ref v) => {
                let mut fmt_string = String::new();
                fmt_string.push('[');
                for (i, o) in v.iter().enumerate() {
                    fmt_string.push_str(format!("{}", o).as_str());
                    if i < v.len() - 1 {
                        fmt_string.push_str(", ");
                    }
                }
                fmt_string.push(']');
                write!(f, "{}", fmt_string)
            }
            Object::Function(_, _, _) => write!(f, "[function]"),
            Object::Builtin(ref name, _, _) => write!(f, "[built-in function: {}]", *name),

            Object::Hash(ref hashmap) => {
                let mut fmt_string = String::new();
                fmt_string.push('{');
                for (i, (k, v)) in hashmap.iter().enumerate() {
                    fmt_string.push_str(format!("{} : {}", k, v.clone()).as_str());
                    if i < hashmap.len() - 1 {
                        fmt_string.push_str(", ");
                    }
                }
                fmt_string.push('}');
                write!(f, "{}", fmt_string)
            }
            Object::Return(ref o) => write!(f, "{}", *o),
            Object::Error(ref s) => write!(f, "Error: {}", s),
        }
    }
}

impl Hash for Object {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match *self {
            Object::Integer(ref i) => i.hash(state),
            Object::Boolean(ref b) => b.hash(state),
            Object::String(ref s) => s.hash(state),
            _ => "".hash(state),
        }
    }
}
