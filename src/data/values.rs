use std::fmt;
use std::collections::BTreeSet;
use std::collections::btree_set::Iter;
use std::ops::Add;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub enum Type {
    Boolean,
    Date,
    Float,
    Int,
    String,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Type::Boolean => write!(f, "bool"),
            Type::Date => write!(f, "date"),
            Type::Float => write!(f, "float"),
            Type::Int => write!(f, "int"),
            Type::String => write!(f, "string"),
        }
    }
}

impl Type {
    pub fn to_str(self) -> &'static str {
        match self {
            Type::Boolean => "bool",
            Type::Date => "date",
            Type::Float => "float",
            Type::Int => "int",
            Type::String => "string",
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Values {
    data_types: BTreeSet<Type>,
}

impl Values {
    pub fn new(data_type: Type) -> Values {
        let mut data_types = BTreeSet::new();
        data_types.insert(data_type);
        Values { data_types }
    }

    #[cfg(test)]
    pub fn from_values(values: &[Type]) -> Values {
        Values {
            data_types: values.iter().map(|t| *t).collect(),
        }
    }

    pub fn len(&self) -> usize {
        self.data_types.len()
    }

    pub fn iter(&self) -> Iter<Type> {
        self.data_types.iter()
    }

    pub fn as_string(&self) -> String {
        let mut string = String::new();
        let mut types = self.data_types.iter();

        if self.data_types.len() > 1 {
            for _ in 0..(self.data_types.len() - 1) {
                string.push_str(types.next().unwrap().to_str());
                string.push_str("|");
            }
        }

        string.push_str(types.next().unwrap().to_str());
        string
    }
}

impl Add for Values {
    type Output = Values;

    fn add(self, other: Values) -> Values {
        Values {
            data_types: self.data_types.union(&other.data_types).cloned().collect(),
        }
    }
}

impl Default for Values {
    fn default() -> Values {
        Values {
            data_types: Default::default(),
        }
    }
}
