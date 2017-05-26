
use serde_json::Number;
use std::collections::HashMap;

use std::ops::Add;

mod values;
pub use self::values::*;

mod objects;
pub use self::objects::*;

mod arrays;
pub use self::arrays::*;

#[cfg(test)]
mod test_data;

#[derive(Debug, PartialEq)]
pub enum Case {
    Values(Values),
    Array(Array),
    Object(Object),
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

    pub fn from_boolean() -> Case {
        Case::Values(Values::new(Type::Boolean))
    }

    pub fn from_string() -> Case {
        Case::Values(Values::new(Type::String))
    }

    pub fn from_dict(dict: HashMap<String, Case>) -> Case {
        Case::Object(Object::from(dict))
    }

    pub fn from_vec(elements: Vec<Case>) -> Case {
        Case::Array(Array::from(elements))
    }
}

impl Add for Case {
    type Output = Case;

    fn add(self, other: Case) -> Case {
        use Case::*;

        match (self, other) {
            (Values(vals_a), Values(vals_b)) => Values(vals_a + vals_b),
            (Null, smt) | (smt, Null) => smt,
            (Object(obj_a), Object(obj_b)) => Object(obj_a + obj_b),
            (Array(arr), Values(vals)) |
            (Values(vals), Array(arr)) => Multi(vec![Array(arr), Values(vals)]),

            (Object(obj), Values(vals)) |
            (Values(vals), Object(obj)) => Multi(vec![Object(obj), Values(vals)]),

            (Array(arr_a), Array(arr_b)) => Case::Array(arr_a + arr_b),
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
