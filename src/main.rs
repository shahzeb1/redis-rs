use std::io::{self, Write};

#[derive(Debug)]
enum Action {
    Get(String),
    Set(String, String),
}

fn do_action(input: &str) {
    let upper = input.to_uppercase();

    let action = match upper.as_str() {
        "GET" => Action::Get("blah".to_string()),
        "SET" => Action::Set("abc".to_string(), "def".to_string()),
        _ => !todo!("Handle unknown command"),
    };

    println!("{:?}", action);
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
