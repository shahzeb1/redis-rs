use crate::actions::get::Get;
use crate::actions::{Action, ActionTrait, DataType, Incr};
use std::collections::HashMap;

mod actions;

// A utility function used to parse user provided string
// into an Action struct which we can call the .execute() func on.
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
                    (Some(k), v) => Ok(Action::SetAction(actions::Set {
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

// The main entrypoint for the library
// Pass in the user input string and the data container
// and this function will execute the action on the data container.
pub fn run(input: &str, data: &mut DataType) {
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

// Creates the hashmap where all the redis key/values will be stored
pub fn create_data_container() -> DataType {
    let data: DataType = HashMap::new();
    return data;
}
