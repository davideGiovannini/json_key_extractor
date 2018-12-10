use structopt::StructOpt;

use std::fs::File;
use std::io::{stdin, Read, Result};

use json_key_extractor::*;

#[cfg(debug_assertions)]
fn debug(message: &str, enabled: bool) {
    if enabled {
        println!("{}", message)
    }
}

fn main() -> Result<()> {
    let args = Args::from_args();

    #[cfg(debug_assertions)]
    debug(&format!("{:#?}", args), args.debug);

    let num_threads = args.num_threads.unwrap_or_else(num_cpus::get);

    let result = if let Some(input_path) = args.input_path {
        let file = File::open(&input_path);
        match file {
            Ok(file) => process(file, num_threads),
            Err(err) => {
                eprintln!("Error while reading '{}': {}", &input_path, err.to_string());
                ::std::process::exit(2)
            }
        }
    } else {
        process(stdin(), num_threads)
    };

    #[cfg(debug_assertions)]
    println!("Beginning printing phase");

    match args.format {
        Printer::Scala => {
            ScalaPrinter::new(args.color).write(&mut std::io::stdout(), &result, args.color)
        }
        Printer::Terminal => {
            TerminalPrinter::default().write(&mut std::io::stdout(), &result, args.color)
        }
    }
}

fn process<Source: Read + Sized>(input: Source, nthreads: usize) -> Case
where
    Source: Read,
{
    if nthreads > 1 {
        #[cfg(debug_assertions)]
        println!("Starting parallel processing [{} threads].", nthreads);

        parallel_process_input(input, nthreads)
    } else {
        #[cfg(debug_assertions)]
        println!("Starting processing [single thread].");

        process_input(input)
    }
}
