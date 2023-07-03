use super::CasePrinter;
use crate::data::{Array, Case, Object};
use std::io::Write;

use ansi_term::{
    Colour::{Green, Yellow},
    Style,
};

use crate::app::ColorOption;
use crate::printers::code::language::Language;

mod class;
pub mod language;

#[cfg(test)]
mod test;

use self::class::Class;

const DEFAULT_CLASS_NAME: &str = "RenameMe";
const DEFAULT_UNKNOWN_TYPE: &str = "UnknownType";

pub struct CodePrinter {
    classes: Vec<Class>,
    language: Box<dyn Language>,
    style: ScalaStyle,
}

impl CasePrinter for CodePrinter {
    fn write<W>(&mut self, writer: &mut W, case: &Case, _: ColorOption) -> crate::Result<()>
    where
        W: Write,
    {
        write!(writer, "{}", self.language.declarations())?;

        match case {
            Case::Values(_) => {
                //TODO
            }
            Case::Array(array) => {
                writeln!(
                    writer,
                    "type {} = {};\n",
                    DEFAULT_CLASS_NAME,
                    self.extract_array(array, None)?
                )?;
            }
            Case::Object(object) => {
                self.extract_classes(object, None)?;
            }
            Case::Null => {
                //TODO
            }
        }

        for class in &self.classes {
            class.write(writer, self.language.as_ref(), &self.style)?;
        }

        Ok(())
    }
}

impl CodePrinter {
    #[must_use]
    pub fn new(language: Box<dyn Language>, color_option: ColorOption) -> Self {
        CodePrinter {
            classes: Vec::default(),
            language,
            style: ScalaStyle::new(color_option.should_use_color()),
        }
    }

    fn extract_classes(&mut self, object: &Object, name: Option<&str>) -> crate::Result<()> {
        let mut class = Class::default();

        class
            .name
            .push_str(&title_case(name.unwrap_or(DEFAULT_CLASS_NAME)));

        for (key, value) in object.values() {
            match value {
                Case::Null => {}
                Case::Values(ref values) => {
                    if values.len() == 1 {
                        let val = values.get_value().unwrap();
                        class.fields.insert(
                            key.clone(),
                            self.style
                                .types
                                .paint(&self.language.type_to_string(val))
                                .to_string(),
                        );
                    } else {
                        // TODO try to handle mixed cases
                    }
                }
                Case::Array(ref array) => {
                    class
                        .fields
                        .insert(key.clone(), self.extract_array(array, Some(key))?);
                }
                Case::Object(ref object) => {
                    class.fields.insert(
                        key.clone(),
                        self.style.types_bold.paint(title_case(key)).to_string(),
                    );
                    self.extract_classes(object, Some(key))?;
                }
            }
        }
        self.classes.push(class);
        Ok(())
    }

    fn extract_array(&mut self, array: &Array, field_name: Option<&str>) -> crate::Result<String> {
        let array_type = self.style.types.paint(self.language.array_string());
        let array_brackets = self.language.array_brackets();

        // TODO try to handle mixed cases (more than one type of values (values, array, object)
        if let Some(values) = array.values() {
            if values.len() == 1 {
                let val = values.get_value().unwrap();

                Ok(format!(
                    "{}{}{}{}",
                    array_type,
                    array_brackets[0],
                    self.style
                        .types_italic
                        .paint(&self.language.type_to_string(val)),
                    array_brackets[1]
                ))
            } else {
                // TODO try to handle mixed cases
                todo!()
            }
        } else if let Some(object) = array.object() {
            self.extract_classes(object, field_name)?;
            Ok(format!(
                "{}{}{}{}",
                array_type,
                array_brackets[0],
                self.style
                    .types_bold
                    .paint(title_case(field_name.unwrap_or(DEFAULT_CLASS_NAME))),
                array_brackets[1]
            ))
        } else if let Some(array) = array.array() {
            Ok(format!(
                "{}{}{}{}",
                array_type,
                array_brackets[0],
                self.extract_array(array, field_name)?,
                array_brackets[1]
            ))
        } else {
            Ok(format!(
                "{}{}{}{}",
                array_type, array_brackets[0], DEFAULT_UNKNOWN_TYPE, array_brackets[1]
            ))
        }
    }
}

fn title_case(string: &str) -> String {
    let mut new_string = String::with_capacity(string.len());

    let mut chars = string.chars();

    if let Some(ch) = chars.next() {
        new_string.push_str(&ch.to_uppercase().to_string());
    }

    for c in chars {
        new_string.push(c);
    }
    new_string
}

#[derive(Default)]
pub struct ScalaStyle {
    keyword: Style,
    types: Style,
    types_italic: Style,
    types_bold: Style,
}

impl ScalaStyle {
    fn new(use_color: bool) -> Self {
        if use_color {
            ScalaStyle {
                keyword: Yellow.normal(),
                types: Green.normal(),
                types_italic: Green.normal().italic(),
                types_bold: Green.normal().bold(),
            }
        } else {
            ScalaStyle::default()
        }
    }
}
