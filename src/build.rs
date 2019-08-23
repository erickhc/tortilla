use tortilla::compiler;
use tortilla::contract::Contract;
use termion::color;
use super::config::Config;
use notify::{RecommendedWatcher, Watcher, RecursiveMode, DebouncedEvent};
use std::sync::mpsc::channel;
use std::time::Duration;
use std::io::Result;
use std::path::Path;
use chrono::prelude::*;

pub fn build(config: &Config) -> Result<Vec<Contract>> {
    let contracts = compiler::compile_paths(&config.inputs)?;

    if config.output == "-" {
        for c in contracts.iter() {
            println!("{}", c.pretty_print());
        }
    } else {
        print_compiled_contracts(&contracts);
        if config.output != "" {
            for c in contracts.iter() {
                c.write_to_dir(&config.output)?;
            }
        }
    }

    Ok(contracts)
}

pub fn watch(config: &Config) -> notify::Result<()> {
    build(config).unwrap();

    let inputs = &config.inputs;
    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_millis(500))?;

    for input in inputs.iter() {
        watcher.watch(&input, RecursiveMode::Recursive)?;
    }

    loop {
        match rx.recv() {
            Ok(DebouncedEvent::Create(_))
            | Ok(DebouncedEvent::Write(_)) => {
                &build(config).unwrap();
            },
            Ok(DebouncedEvent::NoticeRemove(path)) => {
                if inputs.contains(&path) {
                    if let Err(err) = reattach_watcher_file(&mut watcher, path) {
                        eprintln!("{}{:?}{}", color::Fg(color::Red), err, color::Fg(color::Reset));
                    }
                }
            },
            Err(e) => eprintln!("{}{:?}{}", color::Fg(color::Red), e, color::Fg(color::Reset)),
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
    let local = Local::now();
    for c in contracts.iter() {
        println!("[{}] {}{} compiled{}",
            local.format("%Y-%m-%d %H:%M:%S").to_string(),
            color::Fg(color::Green),
            c.name,
            color::Fg(color::Reset)
        );
    }
}
