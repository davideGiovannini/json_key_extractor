use super::CasePrinter;
use std::io::{Result, Write};
use data::{Case, Object};

use ansi_term::{Colour::{Green, Yellow}, Style};

use app::ColorOption;

mod class;
use self::class::*;

pub struct ScalaPrinter {
    classes: Vec<ScalaClass>,
    style: ScalaStyle,
}

impl CasePrinter for ScalaPrinter {
    fn write<W>(&mut self, writer: &mut W, case: &Case, _: ColorOption) -> Result<()>
    where
        W: Write,
    {
        if let Case::Object(ref object) = case {
            self.extract_classes(object, "RenameMe")?;
        } else {
            // TODO decide what to do in the other cases
        }

        for class in &self.classes {
            class.write(writer, &self.style)?;
        }

        Ok(())
    }
}

impl ScalaPrinter {
    pub fn new(color_option: ColorOption) -> Self {
        ScalaPrinter {
            classes: Vec::default(),
            style: ScalaStyle::new(color_option.should_use_color()),
        }
    }

    fn extract_classes(&mut self, object: &Object, name: &str) -> Result<()> {
        let mut class = ScalaClass::default();

        class.name.push_str(&title_case(name));

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
                                .paint(title_case(&to_scala_type(val)))
                                .to_string(),
                        );
                    }
                }
                Case::Array(ref array) => {
                    // TODO try to handle mixed cases
                    if let Some(values) = array.values() {
                        if values.len() == 1 {
                            let val = values.get_value().unwrap();

                            class.fields.insert(
                                key.clone(),
                                format!(
                                    "{}[{}]",
                                    self.style.types.paint("List"),
                                    self.style
                                        .types
                                        .italic()
                                        .paint(title_case(&to_scala_type(val)))
                                ),
                            );
                        }
                    } else if let Some(object) = array.object() {
                        class.fields.insert(
                            key.clone(),
                            format!(
                                "{}[{}]",
                                self.style.types.paint("List"),
                                self.style.types.bold().paint(title_case(key))
                            ),
                        );
                        self.extract_classes(object, key)?;
                    }
                }
                Case::Object(ref object) => {
                    class.fields.insert(
                        key.clone(),
                        self.style.types.bold().paint(title_case(key)).to_string(),
                    );
                    self.extract_classes(object, key)?;
                }
            }
        }
        self.classes.push(class);
        Ok(())
    }
}

fn title_case(string: &str) -> String {
    let mut new_string = String::with_capacity(string.len());

    let mut chars = string.chars();

    if let Some(ch) = chars.next() {
        new_string.push_str(&ch.to_uppercase().to_string())
    }

    for c in chars {
        new_string.push(c)
    }
    new_string
}

pub struct ScalaStyle {
    keyword: Style,
    types: Style,
}

impl ScalaStyle {
    fn new(use_color: bool) -> Self {
        let keyword = if use_color {
            Yellow.normal()
        } else {
            Style::default()
        };
        let types = if use_color {
            Green.normal()
        } else {
            Style::default()
        };
        ScalaStyle { keyword, types }
    }
}
