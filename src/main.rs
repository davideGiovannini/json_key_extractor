extern crate argparse;
extern crate json_key_extractor;

use argparse::{ArgumentParser, StoreOption, Store};
use std::fs::File;
use std::io::{Read, stdin};

use json_key_extractor::*;

fn main() {
    let mut num_threads = 1;
    let mut input_path: Option<String> = None;
    {
        // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("Extract structure information from a jsonl file.");
        ap.refer(&mut num_threads)
            .add_option(&["-n", "--nthreads"], Store, "Number of threads");
        ap.refer(&mut input_path)
            .add_argument("file",
                          StoreOption,
                          "File to process, if not provided stdin will be used.");
        ap.parse_args_or_exit();
    }

    let result = if input_path.is_none() {
        process(stdin(), num_threads)
    } else {
        process(File::open(input_path.unwrap()).unwrap(), num_threads)
    };
    println!("{}", result);
}

fn process<Source: Read + Sized>(input: Source, nthreads: usize) -> String
    where Source: Read
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
