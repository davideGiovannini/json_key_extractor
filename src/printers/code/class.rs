use std::collections::BTreeMap;

use super::ScalaStyle;
use crate::printers::code::language::Language;
use std::io::{Result, Write};

#[derive(Default, Debug)]
pub struct Class {
    pub name: String,
    pub fields: BTreeMap<String, String>,
}

impl Class {
    pub fn write<W>(
        &self,
        writer: &mut W,
        language: &dyn Language,
        style: &ScalaStyle,
    ) -> Result<()>
    where
        W: Write,
    {
        writeln!(writer, "{}", language.start_object(&self.name))?;

        let fields = self
            .fields
            .iter()
            .map(|(n, f)| format!("    {}: {}", style.keyword.paint(n), f))
            .collect::<Vec<String>>()
            .join(",\n");

        writeln!(writer, "{fields}")?;
        writeln!(writer, "{}", language.end_object())
    }
}
