use structopt::StructOpt;

use std::fs::File;
use std::io::{stdin, Read};

use json_key_extractor::*;

fn main() -> Result<()> {
    human_panic::setup_panic!();

    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Debug)
        .format_timestamp(None)
        .init();

    let args = Args::from_args();

    log::debug!("{:#?}", args);

    let num_threads = args.num_threads.unwrap_or_else(num_cpus::get);

    let result = if let Some(input_path) = args.input_path {
        let file = File::open(&input_path);
        match file {
            Ok(file) => process_with(file, num_threads),
            Err(err) => {
                log::error!("Error while reading '{}': {}", &input_path, err.to_string());
                ::std::process::exit(2)
            }
        }
    } else {
        process_with(stdin(), num_threads)
    }?;

    log::debug!("Beginning printing phase");

    match args.format {
        Printer::Scala => {
            ScalaPrinter::new(args.color).write(&mut std::io::stdout(), &result, args.color)
        }
        Printer::Terminal => {
            TerminalPrinter::default().write(&mut std::io::stdout(), &result, args.color)
        }
    }
}

fn process_with<Source: Read + Sized>(input: Source, nthreads: usize) -> Result<Case> {
    if nthreads > 1 {
        log::info!("Starting parallel processing [{} threads].", nthreads);

        parallel_process(input, nthreads)
    } else {
        log::info!("Starting processing [single thread].");

        process(input)
    }
}
