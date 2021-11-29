use std::fmt;
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

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Values {
    bool: Option<Type>,
    date: Option<Type>,
    float: Option<Type>,
    int: Option<Type>,
    string: Option<Type>,
}

impl Values {
    pub fn new(data_type: Type) -> Values {
        let mut values = Values::default();
        match data_type {
            Type::Boolean => values.bool = Some(data_type),
            Type::Date => values.date = Some(data_type),
            Type::Float => values.float = Some(data_type),
            Type::Int => values.int = Some(data_type),
            Type::String => values.string = Some(data_type),
        }
        values
    }

    #[cfg(test)]
    pub fn from_values(values: &[Type]) -> Values {
        let mut res_values = Values::default();

        for data_type in values {
            match data_type {
                Type::Boolean => res_values.bool = Some(*data_type),
                Type::Date => res_values.date = Some(*data_type),
                Type::Float => res_values.float = Some(*data_type),
                Type::Int => res_values.int = Some(*data_type),
                Type::String => res_values.string = Some(*data_type),
            }
        }
        res_values
    }

    pub fn len(&self) -> usize {
        let mut size = 0;
        if self.bool.is_some() {
            size += 1;
        }
        if self.date.is_some() {
            size += 1;
        }
        if self.float.is_some() {
            size += 1;
        }
        if self.int.is_some() {
            size += 1;
        }
        if self.string.is_some() {
            size += 1;
        }
        size
    }

    pub fn to_vec(&self) -> Vec<Type> {
        [self.bool, self.date, self.float, self.int, self.string]
            .iter()
            .copied()
            .flatten()
            .collect()
    }

    pub fn get_value(&self) -> Option<Type> {
        self.bool
            .or(self.date)
            .or(self.int)
            .or(self.float)
            .or(self.string)
    }
}

impl Add for Values {
    type Output = Values;

    fn add(self, other: Values) -> Values {
        Values {
            bool: self.bool.or(other.bool),
            date: self.date.or(other.date),
            float: self.float.or(other.float),
            int: self.int.or(other.int),
            string: self.string.or(other.string),
        }
    }
}
