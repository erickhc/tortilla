use tortilla::compiler;
use tortilla::contract::Contract;
use termion::{color, screen, clear, cursor};
use super::config::Config;
use notify::{RecommendedWatcher, Watcher, RecursiveMode, DebouncedEvent};
use std::sync::mpsc::channel;
use std::time::Duration;
use std::io::{Result, stdout, Write};
use std::path::Path;
use chrono::prelude::*;

pub fn build(config: &Config) -> Result<Vec<Contract>> {
    let contracts = compiler::compile_paths(&config.inputs)?;

    if config.output == "-" {
        for c in contracts.iter() {
            if config.pretty_print {
                println!("{}", c.pretty_print());
            } else {
                println!("{}", c);
            }
        }
    } else {
        print_compiled_contracts(&contracts, config.gas);
        if config.output != "" {
            for c in contracts.iter() {
                c.write_to_dir(&config.output, config.pretty_print)?;
            }
        }
    }

    Ok(contracts)
}

pub fn watch(config: &Config) -> notify::Result<()> {
    let _altscreen = screen::AlternateScreen::from(stdout());
    restart_screen().unwrap();
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
                restart_screen().unwrap();
                &build(config).unwrap();
            },
            Ok(DebouncedEvent::NoticeRemove(path)) => {
                if inputs.iter().any(|x| path.ends_with(x)) {
                    if let Err(err) = reattach_watcher_file(&mut watcher, &path) {
                        eprintln!("{}{:?}{}", color::Fg(color::Red), err, color::Fg(color::Reset));
                    }
                }
            },
            Err(e) => eprintln!("{}{:?}{}", color::Fg(color::Red), e, color::Fg(color::Reset)),
            _ => {},
        }
    }
}

fn restart_screen() -> Result<()> {
    print!("{}", clear::All);
    print!("{}", cursor::Goto(1, 1));
    stdout().flush()?;
    Ok(())
}

fn reattach_watcher_file(watcher: &mut RecommendedWatcher, file: impl AsRef<Path>) -> notify::Result<()> {
    watcher.unwatch(file.as_ref())?;
    watcher.watch(file.as_ref(), RecursiveMode::Recursive)?;
    Ok(())
}

fn print_compiled_contracts(contracts: &[Contract], gas_estimates: bool) {
    let local = Local::now();
    for c in contracts.iter() {
        println!("[{}] {}{} compiled{}",
            local.format("%Y-%m-%d %H:%M:%S").to_string(),
            color::Fg(color::Green),
            c.name,
            color::Fg(color::Reset)
        );
        if gas_estimates {
            println!("{}", c.gas_estimates_to_string());
        }
    }
}
