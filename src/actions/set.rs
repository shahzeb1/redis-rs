use super::{ActionTrait, DataType, Value};

pub struct Set {
    pub key: String,
    pub value: String,
}

impl ActionTrait for Set {
    fn execute(&self, data: &mut DataType) {
        let key = self.key.clone();
        let value = self.value.clone();
        if let Ok(int_value) = value.parse::<i32>() {
            data.insert(key, Value::Int(int_value));
        } else {
            data.insert(key, Value::Str(value));
        }
        println!("OK");
    }
}
