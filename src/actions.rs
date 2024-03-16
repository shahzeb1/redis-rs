use std::collections::HashMap;
use std::fmt;

pub mod get;
pub mod set;
use get::Get;
use set::Set;

// This value enum is only used in the HashMap
pub enum Value {
    Str(String),
    Int(i32),
}

// DataType of the HashMap
pub type DataType = HashMap<String, Value>;

pub struct Incr(pub String);

pub enum Action {
    GetAction(Get),
    SetAction(Set),
    IncrAction(Incr),
}

pub trait ActionTrait {
    fn execute(&self, data: &mut DataType);
}

// These should be their own files:

impl ActionTrait for Incr {
    fn execute(&self, data: &mut DataType) {
        if let Some(value) = data.get_mut(&self.0) {
            if let Value::Int(int_value) = value {
                *int_value += 1;
                println!("(integer) {}", int_value);
            } else {
                println!("Value is not an integer or out of range");
            }
        } else {
            data.insert(self.0.clone(), Value::Int(1));
            println!("(integer) {}", 1);
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Str(s) => write!(f, "\"{}\"", s),
            Value::Int(i) => write!(f, "\"{}\"", i),
        }
    }
}
