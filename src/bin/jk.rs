use structopt::StructOpt;

use log::{debug, error, info};
use std::fs::File;
use std::io::{stdin, Read, Result};

use json_key_extractor::*;

fn main() -> Result<()> {
    human_panic::setup_panic!();

    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Debug)
        .format_timestamp(None)
        .init();

    let args = Args::from_args();

    debug!("{:#?}", args);

    let num_threads = args.num_threads.unwrap_or_else(num_cpus::get);

    let result = if let Some(input_path) = args.input_path {
        let file = File::open(&input_path);
        match file {
            Ok(file) => process(file, num_threads),
            Err(err) => {
                error!("Error while reading '{}': {}", &input_path, err.to_string());
                ::std::process::exit(2)
            }
        }
    } else {
        process(stdin(), num_threads)
    };

    debug!("Beginning printing phase");

    match args.format {
        Printer::Scala => {
            ScalaPrinter::new(args.color).write(&mut std::io::stdout(), &result, args.color)
        }
        Printer::Terminal => {
            TerminalPrinter::default().write(&mut std::io::stdout(), &result, args.color)
        }
    }
}

fn process<Source: Read + Sized>(input: Source, nthreads: usize) -> Case {
    if nthreads > 1 {
        info!("Starting parallel processing [{} threads].", nthreads);

        parallel_process_input(input, nthreads)
    } else {
        info!("Starting processing [single thread].");

        process_input(input)
    }
}
