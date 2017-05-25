
use serde_json::Number;
use std::collections::{HashMap, HashSet};

use std::ops::Add;

mod values;
pub use self::values::*;


#[cfg(test)]
mod test_data;

#[derive(Debug, PartialEq)]
pub enum Case {
    Values(Values),
    Array(Vec<Case>),
    Object(HashMap<String, Case>),
    Null,
    Multi(Vec<Case>),
}

impl Case {
    pub fn from_number(number: Number) -> Case {
        if number.is_f64() {
            Case::Values(Values::new(Type::Float))
        } else {
            Case::Values(Values::new(Type::Int))
        }
    }

    pub fn from_boolean() -> Case{
        Case::Values(Values::new(Type::Boolean))
    }

    pub fn from_string() -> Case{
        Case::Values(Values::new(Type::String))
    }

    pub fn new_values(value_a: Type, value_b: Type)-> Case{
        let mut hashset =  HashSet::with_capacity(2);
        hashset.insert(value_a);
        hashset.insert(value_b);
        Case::Values(Values::from_values(&[value_a, value_b]))
    }
}

impl Add for Case {
    type Output = Case;

    fn add(self, other: Case) -> Case {
        use Case::*;

        match (self, other) {
            (Values(vals_a), Values(vals_b)) => Values(vals_a+vals_b),
            (Null, smt) | (smt, Null) => smt,
            (Object(obj_a), Object(obj_b)) => merge_objects(obj_a, obj_b),
            (Array(arr), Values(vals)) |
            (Values(vals), Array(arr)) => Multi(vec![Array(arr), Values(vals)]),

            (Object(obj), Values(vals)) |
            (Values(vals), Object(obj)) => Multi(vec![Object(obj), Values(vals)]),

            (Array(arr_a), Array(arr_b)) => merge_arrays(arr_a, arr_b),
            (Array(arr), Object(obj)) |
            (Object(obj), Array(arr)) => Multi(vec![Object(obj), Array(arr)]),
            (Multi(mut multi_a), Multi(multi_b)) => {
                multi_a.extend(multi_b);
                Multi(multi_a)
            }
            (Multi(mut multi_a), smt) |
            (smt, Multi(mut multi_a)) => {
                multi_a.push(smt);
                Multi(multi_a)
            }
        }
    }
}



fn merge_objects(mut obj_a: HashMap<String, Case>, obj_b: HashMap<String, Case>) -> Case {
    for (k, v) in obj_b {
        if obj_a.contains_key(&k){
            let a_val = obj_a.remove(&k).unwrap();
            obj_a.insert(k, a_val + v);
        }else{
            obj_a.insert(k, v);
        }
    }
    Case::Object(obj_a)
}
fn merge_arrays(mut arr_a: Vec<Case>, arr_b: Vec<Case>) -> Case {
    arr_a.extend(arr_b);

    compact_array(arr_a)
}


pub fn compact_array(arr: Vec<Case>) -> Case {
    use Case::*;

    let arr = match arr.into_iter().fold(Case::Null, Case::add) {
        Multi(arr) | Array(arr) => arr,
        smt => vec![smt],
    };
    Case::Array(arr)
}
