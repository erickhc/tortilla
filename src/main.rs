use tortilla::compiler;
use clap::{Arg, App};
use std::path::Path;
use std::fs::File;

fn main() -> std::io::Result<()> {
    let matches = App::new("Tortilla")
        .version("0.1.0")
        .author("Erick Hdez <Erick.HernandezCuriel@mx.bosch.com>")
        .about("Solidity compiler")
        .arg(Arg::with_name("INPUT")
             .help("Sets the input file/dir to use")
             .required(true)
             .index(1))
        .get_matches();

    if let Some(input) = matches.value_of("INPUT") {
        let input = Path::new(input);
        if !input.exists() {
            eprintln!("The provided input file/dir does not exist");
            std::process::exit(1);
        }

        let contracts = if input.is_file() {
            let mut file = File::open(input)?;
            compiler::compile_file(&mut file)?
        } else {
            compiler::compile_dir(&input)?
        };

        for c in contracts.into_iter() {
            println!("{}", c.pretty_print());
        }
    }

    Ok(())
}
