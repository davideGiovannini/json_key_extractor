use crate::data::Case;
use serde_json::Value;
use std::collections::BTreeMap;

pub fn process_element(value: Value) -> Case {
    match value {
        Value::Object(map) => {
            let mut object_map: BTreeMap<String, Case> = BTreeMap::default();

            for (key, value) in map {
                let children = process_element(value);
                object_map.insert(key, children);
            }
            Case::from_dict(object_map)
        }
        Value::Null => Case::Null,
        Value::Bool(_) => Case::from_boolean(),
        Value::Number(number) => Case::from_number(&number),
        Value::String(string) => Case::from_string(&string),
        Value::Array(values) => values.into_iter().map(process_element).collect(),
    }
}
