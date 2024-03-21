use crate::actions::{parse_action_from_user_string, Action, ActionTrait, DataType};
use std::collections::HashMap;

mod actions;
mod lib_wasm;

pub struct ShellRunnerContainer {
    data: DataType,
}

// The only reason I created this trait was so that
// the WASM runner could reuse the same methods.
// Read more about why this didn't get used in lib_wasm.rs
pub trait ShellRunner {
    fn new() -> Self;
    fn run(&mut self, input: &str);
}

impl ShellRunner for ShellRunnerContainer {
    fn new() -> Self {
        let data = create_data_container();
        return ShellRunnerContainer { data };
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
