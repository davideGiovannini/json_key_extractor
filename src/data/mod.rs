use serde_json::Number;
use std::collections::BTreeMap;

use std::iter::FromIterator;
use std::ops::Add;

use regex;

mod values;
pub use self::values::*;

mod objects;
pub use self::objects::*;

mod arrays;
pub use self::arrays::*;

#[cfg(test)]
mod test_data;

// This matches both simple dates like YYYY.MM.DD and timestamp like 2017-05-21T08:48:34.943983
const DATE_PATTERN: &str =
    r"^ *\d{4}[-_\\/.:+ ]\d{2}[-_\\/.:+ ]\d{2}(T\d{2}:\d{2}:\d{2}(.\d{6})?)? *$";

#[derive(Debug, PartialEq, Clone)]
pub enum Case {
    Values(Values),
    Array(Array),
    Object(Object),
    Null,
}

impl Case {
    pub fn from_number(number: &Number) -> Case {
        if number.is_f64() {
            Case::Values(Values::new(Type::Float))
        } else {
            Case::Values(Values::new(Type::Int))
        }
    }

    pub fn from_boolean() -> Case {
        Case::Values(Values::new(Type::Boolean))
    }

    pub fn from_string(string: &str) -> Case {
        lazy_static! {
            static ref DATE_REGEXP: regex::Regex =
                    regex::Regex::new(DATE_PATTERN).unwrap();
        }
        let data_type = if DATE_REGEXP.is_match(string) {
            Type::Date
        } else {
            Type::String
        };
        Case::Values(Values::new(data_type))
    }

    pub fn from_dict(dict: BTreeMap<String, Case>) -> Case {
        Case::Object(Object::from(dict))
    }

    pub fn is_object(&self) -> bool {
        if let Case::Object(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_array(&self) -> bool {
        if let Case::Array(_) = self {
            true
        } else {
            false
        }
    }
}

impl FromIterator<Case> for Case {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Case>,
    {
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

            (Array(arr), Object(obj)) | (Object(obj), Array(arr)) => {
                Case::from_iter(vec![Object(obj), Array(arr)])
            }

            (Object(obj), Values(vals)) | (Values(vals), Object(obj)) => {
                Case::from_iter(vec![Object(obj), Values(vals)])
            }

            (Array(arr), Values(vals)) | (Values(vals), Array(arr)) => {
                Case::from_iter(vec![Array(arr), Values(vals)])
            }
        }
    }
}
