use std::collections::btree_map;
use std::collections::BTreeMap;
use std::fmt;
use std::ops::Add;

use super::Case;

#[derive(Debug, PartialEq, Clone)]
pub struct Object {
    dict: BTreeMap<String, Case>,
}

impl Default for Object {
    fn default() -> Self {
        Object {
            dict: Default::default(),
        }
    }
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
    pub fn from(dict: BTreeMap<String, Case>) -> Object {
        Object { dict }
    }

    pub fn keys(&self) -> btree_map::Keys<String, Case> {
        self.dict.keys()
    }

    pub fn values(&self) -> btree_map::Iter<String, Case> {
        self.dict.iter()
    }

    pub fn get(&self, key: &str) -> Option<&Case> {
        self.dict.get(key)
    }

    pub fn len(&self) -> usize {
        self.dict.len()
    }
}
