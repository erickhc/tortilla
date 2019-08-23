use tortilla::compiler;
use tortilla::contract::Contract;
use super::config::Config;
use notify::{RecommendedWatcher, Watcher, RecursiveMode, DebouncedEvent};
use std::sync::mpsc::channel;
use std::time::Duration;
use std::io::Result;
use std::path::Path;

pub fn build(config: &Config) -> Result<Vec<Contract>> {
    let contracts = compiler::compile_paths(&config.inputs)?;

    if config.output == "-" {
        for c in contracts.iter() {
            println!("{}", c.pretty_print());
        }
    } else if config.output != "" {
        for c in contracts.iter() {
            c.write_to_dir(&config.output)?;
        }
    }

    Ok(contracts)
}

pub fn watch(config: &Config) -> notify::Result<()> {
    let contracts = build(config).unwrap();
    print_compiled_contracts(&contracts);

    let inputs = &config.inputs;
    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_millis(500))?;

    for input in inputs.into_iter() {
        watcher.watch(&input, RecursiveMode::Recursive)?;
    }

    loop {
        match rx.recv() {
            Ok(DebouncedEvent::Create(_))
            | Ok(DebouncedEvent::Write(_)) => {
                print_compiled_contracts(&build(config).unwrap());
            },
            Ok(DebouncedEvent::NoticeRemove(path)) => {
                if let Err(err) = reattach_watcher_file(&mut watcher, &path) {
                    eprintln!("{:?}", err);
                }
            },
            Err(e) => eprintln!("{:?}", e),
            _ => {},
        }
    }
}

fn reattach_watcher_file(watcher: &mut RecommendedWatcher, file: impl AsRef<Path>) -> notify::Result<()> {
    watcher.unwatch(file.as_ref())?;
    watcher.watch(file.as_ref(), RecursiveMode::Recursive)?;
    Ok(())
}

fn print_compiled_contracts(contracts: &[Contract]) {
    for c in contracts.iter() {
        println!("{} compiled", c.name);
    }
}
