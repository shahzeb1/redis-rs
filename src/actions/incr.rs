use super::{ActionTrait, DataType, Value};

pub struct Incr(pub String);

impl ActionTrait for Incr {
    fn execute(&self, data: &mut DataType) {
        if let Some(value) = data.get_mut(&self.0) {
            if let Value::Int(int_value) = value {
                *int_value += 1;
            }
        } else {
            data.insert(self.0.clone(), Value::Int(1));
        }
    }

    fn print(&self, data: &mut DataType) {
        if let Some(value) = data.get(&self.0) {
            println!("(integer) {}", value);
        } else {
            println!("Value is not an integer or out of range");
        }
    }
}
