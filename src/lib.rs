extern crate serde_json;

#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate crossbeam;

#[cfg(test)]
#[macro_use]
extern crate quickcheck;

use serde_json::Value;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Read;

mod data;
pub use data::Case;

mod parsing;
pub use parsing::process_element;

mod printer;
pub use printer::pretty_print;


pub fn process_input<Source: Read + Sized>(input: Source) -> Case
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

pub fn parallel_process_input<Source: Read + Sized>(input: Source, n_threads: usize) -> Case
    where Source: Read
{
    use std::thread;
    use std::sync::{Arc, RwLock};
    use std::sync::mpsc::channel;
    use crossbeam::sync::MsQueue;


    let input = BufReader::new(input);
    let queue: Arc<MsQueue<String>> = Arc::new(MsQueue::new());
    let stop_processing = Arc::new(RwLock::new(false));
    let (tx, rx) = channel();

    // spin up n_threads
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

    // Send lines to the threads
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
