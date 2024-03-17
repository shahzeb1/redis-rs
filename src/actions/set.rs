use super::{ActionTrait, DataType, ValueType};

pub struct Set {
    pub key: String,
    pub value: String,
}

impl ActionTrait for Set {
    fn execute(&self, data: &mut DataType) {
        let key = self.key.clone();
        let value = self.value.clone();
        if let Ok(int_value) = value.parse::<i32>() {
            data.insert(key, ValueType::Int(int_value));
        } else {
            data.insert(key, ValueType::Str(value));
        }
        println!("OK");
    }
}

#[cfg(test)]
mod tests {
    use crate::actions;
    use crate::create_data_container;

    use actions::*;

    #[test]
    fn test_execute() {
        let mut data = create_data_container();

        let action = Set {
            key: "key1".to_string(),
            value: "4".to_string(),
        };
        action.execute(&mut data);

        match data.get("key1") {
            Some(val) => {
                let comp = *val == rebs::actions::ValueType::Int(4);
                assert!(comp)
            }
            None => assert!(false, "Value of key1 not correct"),
        }
    }
}
