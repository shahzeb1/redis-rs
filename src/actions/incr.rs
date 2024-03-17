use super::{ActionTrait, DataType, Value};

pub struct Incr(pub String);

impl ActionTrait for Incr {
    fn execute(&self, data: &mut DataType) {
        if let Some(value) = data.get_mut(&self.0) {
            if let Value::Int(int_value) = value {
                *int_value += 1;
                println!("(integer) {}", int_value);
            } else {
                println!("Value is not an integer or out of range");
            }
        } else {
            data.insert(self.0.clone(), Value::Int(1));
            println!("(integer) {}", 1);
        }
    }
}
