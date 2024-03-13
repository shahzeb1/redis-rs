use std::collections::HashMap;
use std::fmt;
use std::io::{self, Write};

#[derive(Debug)]
enum Action {
    Get(String),
    Set(String, String),
}

enum Value {
    Str(String),
    Int(i32),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Str(s) => write!(f, "{}", s),
            Value::Int(i) => write!(f, "{}", i),
        }
    }
}

fn do_action(input: &str, data: &mut HashMap<String, Value>) {
    let mut input_iter = input.split_whitespace();
    let command_optional = input_iter.next();
    if let Some(command) = command_optional {
        let upper = command.to_uppercase();

        // Figure out what the user wants to do,
        // and create an Action to represent it
        let action: Action = match upper.as_str() {
            "GET" => match input_iter.next() {
                Some(k) => Action::Get(k.to_string()),
                None => todo!("Handle GET with no key"),
            },
            "SET" => {
                let key = input_iter.next();
                let value = input_iter.collect::<Vec<&str>>().join(" ");
                match (key, value) {
                    (Some(k), v) => Action::Set(k.to_string(), v.to_string()),
                    _ => todo!("Handle SET with no key or value"),
                }
            }
            _ => todo!("Handle unknown command {input}"),
        };

        // Based on the action, mutate or get data
        match action {
            Action::Get(key) => {
                if let Some(value) = data.get(&key) {
                    println!("{}", value);
                } else {
                    println!("Key {key} not found");
                }
            }
            Action::Set(key, value) => {
                if let Ok(int_value) = value.parse::<i32>() {
                    data.insert(key, Value::Int(int_value));
                } else {
                    data.insert(key, Value::Str(value));
                }
                println!("OK")
            }
        }
    } else {
        todo!("No command found")
    }
}

fn main() {
    let mut data: HashMap<String, Value> = HashMap::new();

    // This loop handles user input
    loop {
        print!("> ");
        let _ = io::stdout().flush(); // Flush screen to show >

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input");

        do_action(input.trim(), &mut data);
    }
}
