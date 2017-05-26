
use serde_json::Number;
use std::collections::HashMap;

use std::iter::FromIterator;
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
}

impl FromIterator<Case> for Case {
    fn from_iter<T>(iter: T) -> Self where T: IntoIterator<Item = Case> {
        Case::Array(Array::from_iter(iter))
    }
}

impl Add for Case {
    type Output = Case;

    fn add(self, other: Case) -> Case {
        use Case::*;

        match (self, other) {
            (Null, smt) | (smt, Null) => smt,

            (Values(vals_a), Values(vals_b)) => Values(vals_a + vals_b),

            (Object(obj_a), Object(obj_b)) => Object(obj_a + obj_b),

            (Array(arr_a), Array(arr_b)) => Case::Array(arr_a + arr_b),

            (Array(arr), Object(obj)) |
            (Object(obj), Array(arr)) => Case::from_iter(vec![Object(obj), Array(arr)]),

            (Object(obj), Values(vals)) |
            (Values(vals), Object(obj)) => Case::from_iter(vec![Object(obj), Values(vals)]),

            (Array(arr), Values(vals)) |
            (Values(vals), Array(arr)) => Case::from_iter(vec![Array(arr), Values(vals)]),
        }
    }
}
