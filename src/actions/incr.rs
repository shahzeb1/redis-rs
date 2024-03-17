use super::{ActionTrait, DataType, ValueType};

pub struct Incr(pub String);

impl ActionTrait for Incr {
    fn execute(&self, data: &mut DataType) {
        if let Some(value) = data.get_mut(&self.0) {
            if let ValueType::Int(int_value) = value {
                *int_value += 1;
                println!("(integer) {}", int_value);
            } else {
                println!("Value is not an integer or out of range");
            }
        } else {
            data.insert(self.0.clone(), ValueType::Int(1));
            println!("(integer) {}", 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::create_data_container;

    use super::*;

    #[test]
    fn test_execute_existing_value() {
        let mut data = create_data_container();
        data.insert("count".to_string(), ValueType::Int(5));

        let action = Incr("count".to_string());
        action.execute(&mut data);

        assert_eq!(data.get("count"), Some(&ValueType::Int(6)));
    }

    #[test]
    fn test_execute_new_value() {
        let mut data = create_data_container();

        let action = Incr("count".to_string());
        action.execute(&mut data);

        assert_eq!(data.get("count"), Some(&ValueType::Int(1)));
    }
}
