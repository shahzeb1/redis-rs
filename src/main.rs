use std::collections::HashMap;
use std::fmt;
use std::io::{self, Write};

#[derive(Debug)]
struct Get(String);

struct Set {
    key: String,
    value: String,
}

struct Incr(String);

enum Action {
    GetAction(Get),
    SetAction(Set),
    IncrAction(Incr),
}

enum Value {
    Str(String),
    Int(i32),
}

trait ActionTrait {
    fn execute(&self, data: &mut HashMap<String, Value>);
}

impl ActionTrait for Get {
    fn execute(&self, data: &mut HashMap<String, Value>) {
        if let Some(value) = data.get(&self.0) {
            println!("{}", value);
        } else {
            println!("Key {} not found", self.0);
        }
    }
}

impl ActionTrait for Set {
    fn execute(&self, data: &mut HashMap<String, Value>) {
        let key = self.key.clone();
        let value = self.value.clone();
        if let Ok(int_value) = value.parse::<i32>() {
            data.insert(key, Value::Int(int_value));
        } else {
            data.insert(key, Value::Str(value));
        }
        println!("OK");
    }
}

impl ActionTrait for Incr {
    fn execute(&self, data: &mut HashMap<String, Value>) {
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
            Value::Str(s) => write!(f, "{}", s),
            Value::Int(i) => write!(f, "{}", i),
        }
    }
}

fn parse_action_from_user_string(input: &str) -> Result<Action, &'static str> {
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

fn do_action(input: &str, data: &mut HashMap<String, Value>) {
    let action_result = parse_action_from_user_string(input);
    match action_result {
        Ok(action) => match action {
            Action::GetAction(get_item) => get_item.execute(data),
            Action::SetAction(set_item) => set_item.execute(data),
            Action::IncrAction(incr_item) => incr_item.execute(data),
        },

        Err(e) => println!("(error) I'm sorry, I don't recognize that command. {e:?}"),
    }
}

fn main() {
    let mut data: HashMap<String, Value> = HashMap::new();

    // This loop handles user input
    loop {
        print!("❯ ");
        let _ = io::stdout().flush(); // Flush screen to show >

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input");

        do_action(input.trim(), &mut data);
    }
}
