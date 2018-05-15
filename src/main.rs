extern crate json_key_extractor;

extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use std::fs::File;
use std::io::{stdin, Read};

use json_key_extractor::*;

use structopt::StructOpt;

/// Extract structure information from a jsonl file.
#[derive(StructOpt, Debug)]
#[structopt()]
struct Args {
    /// Number of threads
    #[structopt(short = "n", long = "nthreads", default_value = "1")]
    num_threads: usize,

    /// File to process, if not provided stdin will be used.
    #[structopt()]
    input_path: Option<String>,
}

fn main() {
    let args = Args::from_args();

    let result = if let Some(input_path) = args.input_path {
        process(File::open(input_path).unwrap(), args.num_threads)
    } else {
        process(stdin(), args.num_threads)
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
