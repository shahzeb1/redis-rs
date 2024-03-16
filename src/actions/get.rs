use super::{ActionTrait, DataType};

pub struct Get(pub String);

impl ActionTrait for Get {
    fn execute(&self, data: &mut DataType) {
        if let Some(value) = data.get(&self.0) {
            println!("{}", value);
        } else {
            println!("Key {} not found", self.0);
        }
    }
}
