use serde_json::Value;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;

pub use crate::data::Case;
pub use crate::parsing::process_element;

pub fn process<Source: Read + Sized>(input: Source) -> Case {
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

pub fn parallel_process<Source: Read + Sized>(input: Source, n_threads: usize) -> Case {
    use crossbeam::queue::SegQueue;
    use std::sync::mpsc::channel;
    use std::sync::{Arc, RwLock};
    use std::thread;

    let input = BufReader::new(input);
    let queue: Arc<SegQueue<String>> = Arc::new(SegQueue::new());
    let stop_processing = Arc::new(RwLock::new(false));
    let (tx, rx) = channel();

    // spin up n_threads
    for _ in 0..n_threads {
        let queue = Arc::clone(&queue);
        let tx = tx.clone();
        let stop_processing = Arc::clone(&stop_processing);

        thread::spawn(move || {
            let mut case = Case::Null;
            loop {
                if let Some(line) = queue.pop() {
                    let v: Value = serde_json::from_str(&line).unwrap();
                    let new_case = process_element(v);
                    case = case + new_case;
                } else if *stop_processing.read().unwrap() {
                    break;
                }
            }
            tx.send(case).unwrap();
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
