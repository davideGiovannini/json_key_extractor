#[macro_use]
extern crate structopt_derive;

mod data;
pub use crate::data::Case;

mod parsing;
pub use crate::parsing::process_element;

mod printers;
pub use crate::printers::*;

mod input;
pub use crate::input::{parallel_process_input, process_input};

mod app;
pub use crate::app::*;
