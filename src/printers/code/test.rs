use super::*;
use crate::data::{Type, Values};
use crate::language::{Rust, Scala};
use std::collections::BTreeMap;

fn values(data_type: Type) -> Case {
    Case::Values(Values::new(data_type))
}

fn array(values: impl IntoIterator<Item = Case>) -> Case {
    Case::Array(Array::from_iter(values))
}

fn test_object() -> Case {
    let mut object = BTreeMap::new();
    object.insert("integer".to_string(), values(Type::Int));
    object.insert("float".to_string(), values(Type::Float));
    object.insert("boolean".to_string(), values(Type::Boolean));
    object.insert("string".to_string(), values(Type::String));
    object.insert("date".to_string(), values(Type::Date));

    object.insert("integers".to_string(), array([values(Type::Int)]));
    object.insert(
        "bool_mat".to_string(),
        array([array([values(Type::Boolean)])]),
    );

    Case::Object(Object::from(object))
}

#[test]
fn scala_printer() {
    let mut printer = CodePrinter::new(Box::new(Scala), ColorOption::Never);

    let mut output = Vec::new();
    printer
        .write(&mut output, &test_object(), ColorOption::Never)
        .unwrap();

    let expected = r#"case class RenameMe(
    bool_mat: List[List[Boolean]],
    boolean: Boolean,
    date: Date,
    float: Float,
    integer: Int,
    integers: List[Int],
    string: String
)
"#;

    assert_eq!(std::str::from_utf8(&output).unwrap(), expected);
}

#[test]
fn rust_printer() {
    let mut printer = CodePrinter::new(Box::new(Rust), ColorOption::Never);

    let mut output = Vec::new();
    printer
        .write(&mut output, &test_object(), ColorOption::Never)
        .unwrap();

    let expected = r#"use serde::{Deserialize, Serialize};

pub type Date = String;

#[derive(Debug, Deserialize, Serialize)]
pub struct RenameMe {
    bool_mat: Vec<Vec<bool>>,
    boolean: bool,
    date: Date,
    float: f64,
    integer: i64,
    integers: Vec<i64>,
    string: String
}
"#;

    assert_eq!(std::str::from_utf8(&output).unwrap(), expected);
}
