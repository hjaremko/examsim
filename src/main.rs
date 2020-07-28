use crate::parser::{parse_exam};
use crate::system::ConsoleApp;
use clap::App;
use log::{debug, error, LevelFilter};
use std::io::BufReader;
use std::fs::File;

mod exam;
mod logger;
mod parser;
mod system;

#[macro_use]
extern crate colour;

#[macro_use]
extern crate clap;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from(yaml).get_matches();
    let log_level = match matches.occurrences_of("verbose") {
        0 => LevelFilter::Debug,
        _ => LevelFilter::max()
    };

    logger::init(log_level).unwrap();
    debug!("Log level: {}", log_level);

    let source_file = match matches.value_of("file") {
        None => {
            error!("You must specify source file!");
            std::process::exit(1)
        }
        Some(s) => s,
    };

    debug!("Source filename: {}", source_file);

    let input_stream =
        // InputStream::new(
        BufReader::new(File::open(source_file).expect("aa"))
        ;
    // );

    match parse_exam(input_stream) {
        Ok(exam) => {
            ConsoleApp::new(exam).run();
        }
        Err(e) => {
            error!("{}", e);
            std::process::exit(1)
        }
    }
}
