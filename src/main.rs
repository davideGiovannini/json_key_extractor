extern crate argparse;
extern crate serde_json;

#[macro_use]
extern crate lazy_static;
extern crate regex;

#[cfg(test)]
#[macro_use]
extern crate quickcheck;

use argparse::{ArgumentParser, StoreOption, StoreTrue};
use serde_json::Value;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::io::{Read, stdin};

mod data;
use data::Case;

mod parsing;
use parsing::process_element;

mod printer;
use printer::pretty_print;

fn main() {
    let mut verbose = false;
    let mut input_path: Option<String> = None;
    {
        // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("Greet somebody.");
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue, "Be verbose");
        ap.refer(&mut input_path)
            .add_argument("file", StoreOption, "Name for the greeting");
        ap.parse_args_or_exit();
    }

    let result = if input_path.is_none() {
        process_input(stdin())
    } else {
        process_input(File::open(input_path.unwrap()).unwrap())
    };
    println!("{}", result);
}

fn process_input<Source: Read + Sized>(input: Source) -> String
    where Source: Read
{
    let input = BufReader::new(input);

    let mut case = Case::Null;

    #[cfg(debug_assertions)]
    println!("Starting");

    for line in input.lines() {
        let line = line.unwrap();
        let v: Value = serde_json::from_str(&line).unwrap();
        let new_case = process_element(v);
        case = case + new_case;
    }

    #[cfg(debug_assertions)]
    println!("Beginning printing phase");

    pretty_print(&case, "")
}
