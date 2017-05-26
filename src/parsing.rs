use std::collections::HashMap;
use serde_json::Value;
use data::Case;
use std::iter::FromIterator;

pub fn process_element(value: Value) -> Case {
    match value {
        Value::Object(map) => {
            let mut object_map: HashMap<String, Case> = Default::default();

            for (key, value) in map {
                let children = process_element(value);
                object_map.insert(key, children);
            }
            Case::from_dict(object_map)
        }
        Value::Null => Case::Null,
        Value::Bool(_) => Case::from_boolean(),
        Value::Number(number) => Case::from_number(number),
        Value::String(_) => Case::from_string(),
        Value::Array(values) => {
            Case::from_vec(Vec::from_iter(values.into_iter().map(process_element)))
        }
    }
}
