use std::collections::BTreeMap;

use super::ScalaStyle;
use crate::data::Type;
use std::io::{Result, Write};

#[derive(Default, Debug)]
pub struct ScalaClass {
    pub name: String,
    pub fields: BTreeMap<String, String>,
}

impl ScalaClass {
    pub fn write<W>(&self, writer: &mut W, style: &ScalaStyle) -> Result<()>
    where
        W: Write,
    {
        writeln!(writer, "case class {}(", self.name)?;

        let fields = self
            .fields
            .iter()
            .map(|(n, f)| format!("    {}: {}", style.keyword.paint(n), f))
            .collect::<Vec<String>>()
            .join(",\n");

        writeln!(writer, "{fields}\n)")
    }
}

pub fn to_scala_type(typ: Type) -> String {
    match typ {
        Type::Boolean => "Boolean",
        Type::Date => "Date",
        Type::Float => "Float",
        Type::Int => "Int",
        Type::String => "String",
    }
    .to_string()
}
