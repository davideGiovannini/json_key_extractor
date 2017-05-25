use std::fmt;
use std::collections::HashSet;
use std::ops::Add;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Type {
    Float,
    Int,
    String,
    Boolean,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Type::Float => write!(f, "float"),
            Type::Int => write!(f, "int"),
            Type::String => write!(f, "string"),
            Type::Boolean => write!(f, "bool"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Values{
    data_types: HashSet<Type>
}

impl Values{
    pub fn new(data_type: Type)-> Values{
        let mut data_types = HashSet::new();
        data_types.insert(data_type);
        Values{data_types}
    }
    pub fn from_values(values: &[Type]) -> Values{
        let mut data_types = HashSet::new();
        for t in values{
            data_types.insert(*t);
        }
        Values{data_types}
    }
}

impl fmt::Display for Values {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        for (i,t) in self.data_types.iter().enumerate(){
            write!(f, "{}", t)?;
            if i < self.data_types.len() -1{
                write!(f, "|")?;
            }
        }
        write!(f, "")
    }
}

impl Add for Values {
    type Output = Values;

    fn add(self, other: Values) -> Values {
        Values{
            data_types: self.data_types.union(&other.data_types).cloned().collect()
        }
    }
}
