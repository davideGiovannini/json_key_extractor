use data::Case;

use std::{io, io::Write};

mod terminal;
pub use self::terminal::*;

mod scala;
pub use self::scala::*;
use std::str::FromStr;

use app::ColorOption;

pub trait CasePrinter {
    fn write<W>(
        &mut self,
        writer: &mut W,
        case: &Case,
        color_option: ColorOption,
    ) -> io::Result<()>
    where
        W: Write;
}

#[derive(Debug)]
pub enum Printer {
    Scala,
    Terminal,
}

impl Default for Printer {
    fn default() -> Self {
        Printer::Terminal
    }
}

impl FromStr for Printer {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match s {
            "scala" => Ok(Printer::Scala),
            "terminal" => Ok(Printer::Terminal),
            other => Err(format!("{} is not a valid output type", other)),
        }
    }
}
