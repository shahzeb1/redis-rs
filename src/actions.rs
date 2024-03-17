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
    fn print(&self, data: &DataType);
}

// A way to print out the Values depending on the type of data.
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Str(s) => write!(f, "{}", s),
            Value::Int(i) => write!(f, "{}", i),
        }
    }
}

mod actions {
    macro_rules! print_value {
        ($success:expr, $failure:expr, $value:expr) => {
            if let Some(value) = $value {
                println!($success, value);
            } else {
                println!($failure);
            }
        };
    }

    // Bit of a hack on how to export macros: https://stackoverflow.com/a/31749071
    pub(crate) use print_value;
}

// A utility function used to parse user provided string
// into an Action struct which we can call the .execute() func on.
#[allow(dead_code)]
pub fn parse_action_from_user_string(input: &str) -> Result<Action, &'static str> {
    let mut input_iter = input.split_whitespace();
    let command_optional = input_iter.next();
    if let Some(command) = command_optional {
        let upper = command.to_uppercase();

        // Figure out what the user wants to do,
        // and create an Action to represent it
        return match upper.as_str() {
            "GET" => match input_iter.next() {
                Some(k) => Ok(Action::GetAction(Get(k.to_string()))),
                None => Err("Handle GET with no key"),
            },
            "SET" => {
                let key = input_iter.next();
                let value = input_iter.collect::<Vec<&str>>().join(" ");
                match (key, value) {
                    (Some(k), v) => Ok(Action::SetAction(Set {
                        key: k.to_string(),
                        value: v,
                    })),
                    _ => Err("Handle SET with no key or value"),
                }
            }
            "INCR" => match input_iter.next() {
                Some(k) => Ok(Action::IncrAction(Incr(k.to_string()))),
                None => Err("Handle INCR with no key"),
            },
            _ => Err("Handle unknown command"),
        };
    } else {
        Err("No command found")
    }
}
