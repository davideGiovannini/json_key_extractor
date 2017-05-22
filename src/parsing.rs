use std::collections::HashMap;
use serde_json::Value;
use data::{Case, compact_array, Type};
use std::iter::FromIterator;

pub fn process_element(value: Value) -> Case {
    match value {
        Value::Object(map) => {
            let mut object_map: HashMap<String, Case> = Default::default();

            for (key, value) in map {
                let children = process_element(value);
                object_map.insert(key, children);
            }
            Case::Object(object_map)
        }
        Value::Null => Case::Null,
        Value::Bool(_) => Case::Value(Type::Boolean),
        Value::Number(number) => Case::from_number(number),
        Value::String(_) => Case::Value(Type::String),
        Value::Array(values) => {
            compact_array(Vec::from_iter(values.into_iter().map(process_element)))
        }
    }
}
