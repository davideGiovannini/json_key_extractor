use std::collections::HashMap;
use std::collections::hash_map::Keys;
use std::fmt;
use std::ops::Add;

use super::Case;

#[derive(Debug, PartialEq)]
pub struct Object {
    dict: HashMap<String, Case>,
}


impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "object")
    }
}

impl Add for Object {
    type Output = Object;

    fn add(mut self, other: Object) -> Object {
        for (k, v) in other.dict {
            if self.dict.contains_key(&k) {
                let a_val = self.dict.remove(&k).unwrap();
                self.dict.insert(k, v.add(a_val));
            } else {
                self.dict.insert(k, v);
            }
        }
        Object { dict: self.dict }
    }
}


impl Object {
    pub fn from(dict: HashMap<String, Case>) -> Object {
        Object { dict }
    }

    pub fn keys(&self) -> Keys<String, Case> {
        self.dict.keys()
    }

    pub fn get(&self, key: &str) -> Option<&Case> {
        self.dict.get(key)
    }
}
