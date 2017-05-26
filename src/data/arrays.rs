use std::fmt;
use std::ops::Add;
use std::iter::FromIterator;

use super::Case;
use super::{Object, Values};

#[derive(Debug, PartialEq)]
pub struct Array {
    values: Values,
    object: Object,
    array: Box<Option<Array>>,
}

impl Default for Array {
    fn default() -> Array {
        Array {
            values: Default::default(),
            object: Default::default(),
            array: Box::new(None),
        }
    }
}

impl fmt::Display for Array {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut type_string = String::new();

        if self.values.len() > 0 {
            type_string.push_str(&format!("{}", self.values));
        }

        if self.object.len() > 0 {
            type_string.push_str(&format!("{}", self.object));
        }

        if let Some(ref arr) = *self.array {
            type_string.push_str(&format!("{}", arr));
        }

        write!(f, "[{}]", type_string)
    }
}

impl Add for Array {
    type Output = Array;

    fn add(mut self, mut other: Array) -> Array {
        self.values = self.values + other.values;
        self.object = self.object + other.object;

        if self.array.is_none() {
            self.array = other.array
        } else if other.array.is_none() {

        } else {
            let arr_a = self.array.take().unwrap();
            let arr_b = other.array.take().unwrap();
            self.array = Box::new(Some(arr_a + arr_b));
        }

        self
    }
}

impl Array {
    pub fn from(elements: Vec<Case>) -> Array {
        let mut array: Array = Default::default();
        for case in elements {
            match case {
                Case::Values(vals) => array.values = array.values + vals,
                Case::Object(obj) => array.object = array.object + obj,
                Case::Array(arr) => array = array + arr,
                Case::Null => (),
            }
        }
        array
    }

    pub fn len(&self) -> usize {
        let array_len = if let Some(ref array) = *self.array {
            array.len()
        } else {
            0
        };
        self.values.len() + self.object.len() + array_len
    }

    pub fn has_object(&self) -> bool {
        self.object.len() > 0
    }

    pub fn object(&self) -> &Object {
        &self.object
    }
}

impl FromIterator<Case> for Array {
    fn from_iter<T>(iter: T) -> Self
        where T: IntoIterator<Item = Case>
    {
        let mut array: Array = Default::default();
        for case in iter {
            match case {
                Case::Values(vals) => array.values = array.values + vals,
                Case::Object(obj) => array.object = array.object + obj,
                Case::Array(arr) => {
                    array.array = if let Some(a) = *array.array {
                        Box::new(Some(a + arr))
                    } else {
                        Box::new(Some(arr))
                    }
                }
                Case::Null => (),
            }
        }
        array
    }
}
