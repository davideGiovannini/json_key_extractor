use super::CasePrinter;
use crate::data::Case;
use std::io::{Result, Write};

use ansi_term::{
    Colour::{Green, Yellow},
    Style,
};
use prettytable::{cell, format, row, Table};

use crate::app::ColorOption;

#[cfg(test)]
mod test;

#[derive(Default)]
pub struct TerminalPrinter;

impl CasePrinter for TerminalPrinter {
    fn write<W>(&mut self, writer: &mut W, case: &Case, color_option: ColorOption) -> Result<()>
    where
        W: Write,
    {
        let style = TerminalStyle::new(color_option.should_use_color());

        let mut table = TerminalPrinter::setup_prettytable();

        let mut prefixes = Vec::new();
        TerminalPrinter::process_case(&mut table, case, &style, &mut prefixes);

        table.print(writer).map(|_| ())
    }
}

impl TerminalPrinter {
    fn setup_prettytable() -> Table {
        let mut table = Table::new();
        let format = format::FormatBuilder::new()
            .column_separator('|')
            .borders('|')
            .separators(
                &[format::LinePosition::Top, format::LinePosition::Bottom],
                format::LineSeparator::new('-', '+', '+', '+'),
            )
            .padding(2, 2)
            .build();
        table.set_format(format);
        table
    }

    fn process_case(
        table: &mut Table,
        case: &Case,
        style: &TerminalStyle,
        prefixes: &mut Vec<&str>,
    ) {
        match case {
            Case::Null | Case::Values(_) | Case::Array(_) => {
                // TODO handle array objects and maybe array of array
                let case_type = TerminalPrinter::type_from_case(case, &style.types);
                table.add_row(row![case_type]);
            }
            Case::Object(ref object) => {
                for (key, case) in object.values() {
                    let mut new_prefixes = prefixes.clone();
                    new_prefixes.push(key);

                    let case_type = TerminalPrinter::type_from_case(case, &style.types);

                    table.add_row(row![
                        new_prefixes
                            .iter()
                            .map(|k| style.keyword.paint(*k).to_string())
                            .collect::<Vec<String>>()
                            .join("."),
                        case_type
                    ]);

                    match case {
                        Case::Object(_) => {
                            TerminalPrinter::process_case(table, case, style, &mut new_prefixes);
                        }
                        Case::Array(ref arr) => {
                            if let Some(obj) = arr.object() {
                                new_prefixes.push("[]");
                                TerminalPrinter::process_case(
                                    table,
                                    &Case::Object(obj.clone()),
                                    style,
                                    &mut new_prefixes,
                                );
                            }
                        }
                        _ => {}
                    };
                }
            }
        };
    }

    fn type_from_case(case: &Case, style: &Style) -> String {
        match case {
            Case::Null => style.paint("null").to_string(),
            Case::Values(ref values) => values
                .to_vec()
                .iter()
                .map(|x| style.paint(x.to_str()).to_string())
                .collect::<Vec<String>>()
                .join(" | "),
            Case::Array(array) => {
                let tvalues = array.values().iter().map(|x| {
                    TerminalPrinter::type_from_case(&Case::Values(x.clone()), &style.italic())
                });
                let tarray = array.array().iter().map(|x| {
                    TerminalPrinter::type_from_case(&Case::Array(*x.clone()), &style.italic())
                });
                let tobject = array.object().iter().map(|x| {
                    TerminalPrinter::type_from_case(
                        &Case::Object(x.clone()),
                        &style.bold().italic(),
                    )
                });

                format!(
                    "[{}]",
                    tvalues
                        .chain(tarray)
                        .chain(tobject)
                        .collect::<Vec<String>>()
                        .join(" | ")
                )
            }
            Case::Object(_) => style.bold().paint("object").to_string(),
        }
    }
}

struct TerminalStyle {
    keyword: Style,
    types: Style,
}

impl TerminalStyle {
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
        TerminalStyle { keyword, types }
    }
}
