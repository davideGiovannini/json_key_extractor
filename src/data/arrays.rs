use std::ops::Add;
use std::iter::FromIterator;

use super::Case;
use super::{Object, Values};

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Array {
    values: Option<Values>,
    object: Option<Object>,
    array: Option<Box<Array>>,
}

impl Add for Array {
    type Output = Array;

    fn add(mut self, other: Array) -> Array {
        self.add_maybe_values(other.values);
        self.add_maybe_object(other.object);
        self.add_maybe_array(other.array);

        self
    }
}

impl Array {
    fn add_maybe_object(&mut self, object: Option<Object>) {
        if self.object.is_none() {
            self.object = object;
        } else if object.is_some() {
            self.object = Some(self.object.take().unwrap() + object.unwrap())
        }
    }

    fn add_maybe_values(&mut self, values: Option<Values>) {
        if self.values.is_none() {
            // take whatever values is in other
            self.values = values;
        } else if values.is_some() {
            // both are present
            // add them togheter
            self.values = Some(self.values.take().unwrap() + values.unwrap())
        }
    }

    fn add_maybe_array(&mut self, array: Option<Box<Array>>) {
        if self.array.is_none() {
            self.array = array;
        } else if array.is_some() {
            self.array = Some(Box::new(*self.array.take().unwrap() + *array.unwrap()))
        }
    }

    pub fn len(&self) -> usize {
        let n_values = if let Some(ref values) = self.values {
            values.len()
        } else {
            0
        };

        let n_objects = if let Some(ref object) = self.object {
            object.len()
        } else {
            0
        };
        let n_array = if let Some(ref array) = self.array {
            array.len()
        } else {
            0
        };
        n_values + n_objects + n_array
    }

    pub fn values(&self) -> &Option<Values> {
        &self.values
    }
    pub fn array(&self) -> &Option<Box<Array>> {
        &self.array
    }
    pub fn object(&self) -> &Option<Object> {
        &self.object
    }
}

impl FromIterator<Case> for Array {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Case>,
    {
        let mut array: Array = Default::default();
        for case in iter {
            match case {
                Case::Values(vals) => array.add_maybe_values(Some(vals)),
                Case::Object(obj) => array.add_maybe_object(Some(obj)),
                Case::Array(arr) => array.add_maybe_array(Some(Box::new(arr))),
                Case::Null => (),
            }
        }
        array
    }
}
