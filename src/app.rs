use crate::printers::Printer;
use std::str::FromStr;
use structopt::StructOpt;

/// Extract structure information from a jsonl file.
#[derive(StructOpt, Debug)]
#[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
pub struct Args {
    /// Number of threads (defaults to the number of logical thread available)
    #[structopt(short = "n", long = "nthreads")]
    pub num_threads: Option<usize>,

    /// File to process, if not provided stdin will be used.
    #[structopt()]
    pub input_path: Option<String>,

    /// Default output format. Can be one of (scala, rust, terminal)
    #[structopt(short = "t", long = "type", default_value = "terminal")]
    pub format: Printer,

    /// When to use colors.
    /// Can be one of (never, always, auto)
    #[structopt(short = "c", long = "color", default_value = "auto")]
    pub color: ColorOption,

    #[cfg(debug_assertions)]
    #[structopt(short = "d", long = "debug")]
    /// Enable debug informations
    pub debug: bool,
}

#[derive(Debug, Copy, Clone)]
pub enum ColorOption {
    Always,
    Auto,
    Never,
}

impl FromStr for ColorOption {
    type Err = String;

    fn from_str(string: &str) -> Result<Self, <Self as FromStr>::Err> {
        match string {
            "always" => Ok(ColorOption::Always),
            "auto" => Ok(ColorOption::Auto),
            "never" => Ok(ColorOption::Never),
            other => Err(format!("{other} is not a valid color option")),
        }
    }
}

impl ColorOption {
    #[must_use]
    pub fn should_use_color(self) -> bool {
        match self {
            ColorOption::Always => true,
            ColorOption::Auto => atty::is(atty::Stream::Stdout),
            ColorOption::Never => false,
        }
    }
}
