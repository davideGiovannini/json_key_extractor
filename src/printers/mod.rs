use crate::data::Case;

use std::io::Write;

mod terminal;
pub use self::terminal::*;

mod code;
pub use self::code::*;
use std::str::FromStr;

use crate::app::ColorOption;
use crate::language::{Language, Rust, Scala};

pub trait CasePrinter {
    fn write<W>(
        &mut self,
        writer: &mut W,
        case: &Case,
        color_option: ColorOption,
    ) -> crate::Result<()>
    where
        W: Write;
}

#[derive(Debug, Default)]
pub enum Printer {
    Code(Box<dyn Language>),
    #[default]
    Terminal,
}

impl FromStr for Printer {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match s {
            "scala" => Ok(Printer::Code(Box::new(Scala))),
            "rust" => Ok(Printer::Code(Box::new(Rust))),
            "terminal" => Ok(Printer::Terminal),
            other => Err(format!("{other} is not a valid output type")),
        }
    }
}
