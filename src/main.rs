extern crate json_key_extractor;

extern crate structopt;
#[macro_use]
extern crate structopt_derive;

extern crate num_cpus;

use std::fs::File;
use std::io::{stdin, Read};

use json_key_extractor::*;

use structopt::StructOpt;

/// Extract structure information from a jsonl file.
#[derive(StructOpt, Debug)]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
struct Args {
    /// Number of threads (defaults to the number of logical thread available)
    #[structopt(short = "n", long = "nthreads")]
    num_threads: Option<usize>,

    /// File to process, if not provided stdin will be used.
    #[structopt()]
    input_path: Option<String>,
}

fn main() {
    let args = Args::from_args();

    let num_threads = args.num_threads.unwrap_or_else(num_cpus::get);

    let result = if let Some(input_path) = args.input_path {
        process(File::open(input_path).unwrap(), num_threads)
    } else {
        process(stdin(), num_threads)
    };

    println!("{}", result);
}

fn process<Source: Read + Sized>(input: Source, nthreads: usize) -> String
where
    Source: Read,
{
    let result = if nthreads > 1 {
        #[cfg(debug_assertions)]
        println!("Starting parallel processing [{} threads].", nthreads);

        parallel_process_input(input, nthreads)
    } else {
        #[cfg(debug_assertions)]
        println!("Starting processing [single thread].");

        process_input(input)
    };

    #[cfg(debug_assertions)]
    println!("Beginning printing phase");

    pretty_print(&result, "")
}
