extern crate serde_json;

extern crate ansi_term;
extern crate crossbeam;

#[macro_use]
extern crate lazy_static;
extern crate regex;

#[macro_use]
extern crate prettytable;

#[cfg(test)]
#[macro_use]
extern crate quickcheck;
#[cfg(test)]
extern crate rand;

extern crate structopt;
#[macro_use]
extern crate structopt_derive;

extern crate atty;

mod data;
pub use data::Case;

mod parsing;
pub use parsing::process_element;

mod printers;
pub use printers::*;

mod input;
pub use input::{parallel_process_input, process_input};

mod app;
pub use app::*;
