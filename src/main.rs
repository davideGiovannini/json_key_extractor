extern crate argparse;
extern crate serde_json;

#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate crossbeam;

#[cfg(test)]
#[macro_use]
extern crate quickcheck;

use argparse::{ArgumentParser, StoreOption, Store};
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

use crossbeam::sync::MsQueue;

fn main() {
    let mut num_threads = 1;
    let mut input_path: Option<String> = None;
    {
        // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("Greet somebody.");
        ap.refer(&mut num_threads)
            .add_option(&["-n", "--nthreads"], Store, "Number of threads");
        ap.refer(&mut input_path)
            .add_argument("file", StoreOption, "Name for the greeting");
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

fn process_input<Source: Read + Sized>(input: Source) -> Case
    where Source: Read
{
    let input = BufReader::new(input);

    let mut case = Case::Null;

    for line in input.lines() {
        let line = line.unwrap();
        let v: Value = serde_json::from_str(&line).unwrap();
        let new_case = process_element(v);
        case = case + new_case;
    }
    case
}

fn parallel_process_input<Source: Read + Sized>(input: Source, n_threads: usize) -> Case
    where Source: Read
{
    let input = BufReader::new(input);

    let queue: Arc<MsQueue<String>> = Arc::new(MsQueue::new());

    use std::thread;
    use std::sync::{Arc, RwLock};
    use std::sync::mpsc::channel;

    let stop_processing = Arc::new(RwLock::new(false));
    let (tx, rx) = channel();

    for _ in 0..n_threads {
        let queue = queue.clone();
        let tx = tx.clone();
        let stop_processing = stop_processing.clone();

        thread::spawn(move || {
            let mut case = Case::Null;
            loop {
                if let Some(line) = queue.try_pop() {
                    let v: Value = serde_json::from_str(&line).unwrap();
                    let new_case = process_element(v);
                    case = case + new_case;
                } else if *stop_processing.read().unwrap() {
                    break;
                }
            }
            tx.send(case).unwrap()
        });
    }

    for line in input.lines() {
        let line = line.unwrap();
        queue.push(line.to_string());
    }

    {
        // Signal threads to stop awaiting data
        *stop_processing.write().unwrap() = true;
    }

    // Collect and combine the results
    let mut case = Case::Null;
    for _ in 0..n_threads {
        let new_case = rx.recv().unwrap();
        case = case + new_case;
    }
    case
}
