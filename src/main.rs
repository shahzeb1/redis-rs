use std::collections::HashMap;
use std::io::{self, Write};

#[derive(Debug)]
enum Action {
    Get(String),
    Set(String, String),
}

fn do_action(input: &str, data: &mut HashMap<String, String>) {
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
                let (key, value) = (input_iter.next(), input_iter.next());
                match (key, value) {
                    (Some(k), Some(v)) => Action::Set(k.to_string(), v.to_string()),
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
                data.insert(key, value);
                println!("OK")
            }
        }
    } else {
        todo!("No command found")
    }
}

fn main() {
    let mut data: HashMap<String, String> = HashMap::new();

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
