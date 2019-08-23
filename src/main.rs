mod build;
mod config;

use clap::{Arg, App};
use std::path::Path;
use config::Config;
use build::{build, watch};
use std::io::Result;

fn main() -> Result<()> {
    let matches = App::new("Tortilla")
        .version("0.1.0")
        .author("Erick Hdez <Erick.HernandezCuriel@mx.bosch.com>")
        .about("Solidity compiler")
        .arg(Arg::with_name("INPUTS")
             .help("Sets the input files/dirs to use")
             .required(true)
             .multiple(true))
        .arg(Arg::with_name("WATCH")
             .short("w")
             .long("watch")
             .help("Sets a watcher over the inputs"))
        .arg(Arg::with_name("OUTPUT")
             .short("o")
             .long("output")
             .takes_value(true)
             .help("Sets the output directory"))
        .arg(Arg::with_name("PRETTY_PRINT")
             .short("p")
             .long("pretty")
             .help("Sets the JSON to be pretty printed"))
        .get_matches();

    let inputs = filter_paths(matches.values_of_lossy("INPUTS").unwrap());
    if inputs.len() == 0 {
        std::process::exit(1);
    }

    let should_watch = matches.is_present("WATCH");
    let output = matches.value_of("OUTPUT").unwrap_or("");
    let pretty_print = matches.is_present("PRETTY_PRINT");

    let config = Config::new(&inputs)
        .watch(should_watch)
        .output(output)
        .pretty_print(pretty_print);

    if config.watch {
        watch(&config).unwrap();
    } else {
        build(&config)?;
    }

    Ok(())
}

fn filter_paths(paths: Vec<impl AsRef<Path>>) -> Vec<impl AsRef<Path>> {
    let mut invalid = Vec::new();
    let mut valid = Vec::new();

    for path in paths.into_iter() {
        if !path.as_ref().exists() {
            invalid.push(String::from(path.as_ref().to_string_lossy()));
        } else {
            valid.push(path);
        }
    }

    for p in invalid.into_iter() {
        eprintln!("{}: No such file or directory", p);
    }

    valid
}
