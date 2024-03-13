use std::io::{self, Write};

#[derive(Debug)]
enum Action {
    Get(String),
    Set(String, String),
}

fn do_action(input: &str) {
    let mut input_iter = input.split_whitespace();
    let command_optional = input_iter.next();
    if let Some(command) = command_optional {
        let upper = command.to_uppercase();

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

        println!("{:?}", action);
    } else {
        todo!("No command found")
    }
}

fn main() {
    loop {
        print!("> ");
        let _ = io::stdout().flush(); // Flush screen to show >

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input");

        do_action(input.trim());
    }
}
