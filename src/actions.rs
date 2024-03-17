use get::Get;
use incr::Incr;
use set::Set;
use std::collections::HashMap;
use std::fmt;

pub mod get;
pub mod incr;
pub mod set;

// This value enum is only used in the HashMap.
pub enum Value {
    Str(String),
    Int(i32),
}

// DataType of the HashMap.
pub type DataType = HashMap<String, Value>;

// This is the Action that contains all the data
// that specific action needs to execute.
// TODO: Adding this macro so compiler warning goes away.
#[allow(dead_code)]
pub enum Action {
    GetAction(Get),
    SetAction(Set),
    IncrAction(Incr),
}

// Every new action in the actions/ folder must
// implement this trait.
pub trait ActionTrait {
    // Execute mutates the data
    fn execute(&self, data: &mut DataType);

    // Print the value to the console
    fn print(&self, data: &mut DataType);
}

// A way to print out the Values depending on the type of data.
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Str(s) => write!(f, "\"{}\"", s),
            Value::Int(i) => write!(f, "\"{}\"", i),
        }
    }
}
