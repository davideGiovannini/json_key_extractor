use crate::data::Type;

pub trait Language: std::fmt::Debug {
    /// Imports and language specific headers
    fn declarations(&self) -> String;

    /// Translate json types to Language
    fn type_to_string(&self, typ: Type) -> String;

    /// Begin an object declaration
    fn start_object(&self, name: &str) -> String;

    /// Close the object declaration
    fn end_object(&self) -> String;

    /// Return the correct array type to use. (eg List for Scala, Vec for Rust)
    fn array_string(&self) -> String;

    /// Return the brackets used to delimit the array
    fn array_brackets(&self) -> [char; 2];
}

#[derive(Default, Debug)]
pub struct Scala;

impl Language for Scala {
    fn declarations(&self) -> String {
        String::new()
    }

    fn type_to_string(&self, typ: Type) -> String {
        match typ {
            Type::Boolean => "Boolean",
            Type::Date => "Date",
            Type::Float => "Float",
            Type::Int => "Int",
            Type::String => "String",
        }
        .to_string()
    }

    fn start_object(&self, name: &str) -> String {
        format!("case class {name}(")
    }

    fn end_object(&self) -> String {
        ")".to_string()
    }

    fn array_string(&self) -> String {
        "List".to_string()
    }

    fn array_brackets(&self) -> [char; 2] {
        ['[', ']']
    }
}

#[derive(Default, Debug)]
pub struct Rust;

impl Language for Rust {
    fn declarations(&self) -> String {
        "use serde::{Deserialize, Serialize};\n\npub type Date = String;\n\n".to_string()
    }

    fn type_to_string(&self, typ: Type) -> String {
        match typ {
            Type::Boolean => "bool",
            Type::Date => "Date",
            Type::Float => "f64",
            Type::Int => "i64",
            Type::String => "String",
        }
        .to_string()
    }

    fn start_object(&self, name: &str) -> String {
        format!("#[derive(Debug, Deserialize, Serialize)]\npub struct {name} {{")
    }

    fn end_object(&self) -> String {
        "}".to_string()
    }

    fn array_string(&self) -> String {
        "Vec".to_string()
    }

    fn array_brackets(&self) -> [char; 2] {
        ['<', '>']
    }
}
