use std::fmt;
use std::ops::Add;
use std::ops::Index;

use super::Case;

#[derive(Debug, PartialEq)]
pub struct Array {
    contents: Vec<Case>,
}

impl fmt::Display for Array {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.contents.len() == 1 {
            match self.contents[0] {
                Case::Values(ref t) => return write!(f, "[{}]", t),
                Case::Object(ref obj) => return write!(f, "[{}]", obj),
                _ => (),
            }
        }
        write!(f, "[array]")
    }
}

impl Add for Array {
    type Output = Array;

    fn add(mut self, other: Array) -> Array {
        self.contents.extend(other.contents);
        self.compact();
        Array { contents: self.contents }
    }
}

impl Array {
    pub fn from(elements: Vec<Case>) -> Array {
        let mut array = Array { contents: elements };
        array.compact();
        array
    }

    fn compact(&mut self) {
        use Case::*;

        let arr = match self.contents.drain(0..).fold(Case::Null, Case::add) {
            Array(arr) => arr.contents,
            smt => vec![smt],
        };
        self.contents = arr;
    }

    pub fn len(&self) -> usize {
        self.contents.len()
    }
}

impl Index<usize> for Array {
    type Output = Case;
    fn index(&self, index: usize) -> &Self::Output {
        &self.contents[index]
    }
}
