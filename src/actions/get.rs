use crate::actions::actions::print_value;

use super::{ActionTrait, DataType};

pub struct Get(pub String);

impl ActionTrait for Get {
    fn execute(&self, _: &mut DataType) {
        // The GET action doesn't modify values in data
        // so we can just return.
    }

    fn print(&self, data: &DataType) {
        print_value!("\"{}\"", "(nil)", data.get(&self.0));
    }
}
