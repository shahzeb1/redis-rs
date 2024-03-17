use crate::actions::{parse_action_from_user_string, Action, ActionTrait, DataType};
use std::collections::HashMap;

mod actions;

pub struct RunnerContainer {
    data: DataType,
}

pub trait Runner {
    fn new() -> Self;
    fn run(&mut self, input: &str);
}

impl Runner for RunnerContainer {
    fn new() -> Self {
        let data = create_data_container();
        return RunnerContainer { data };
    }

    fn run(&mut self, input: &str) {
        let action_result = parse_action_from_user_string(input);

        match action_result {
            Ok(action) => match action {
                Action::GetAction(get_item) => get_item.print(&self.data),
                Action::SetAction(set_item) => {
                    set_item.execute(&mut self.data);
                    set_item.print(&self.data);
                }
                Action::IncrAction(incr_item) => {
                    incr_item.execute(&mut self.data);
                    incr_item.print(&self.data);
                }
            },

            Err(e) => println!("(error) I'm sorry, I don't recognize that command. {e:?}"),
        }
    }
}

// Creates the hashmap where all the redis key/values will be stored
pub fn create_data_container() -> DataType {
    let data: DataType = HashMap::new();
    return data;
}
