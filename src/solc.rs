use crate::abi::*;
use crate::contract::GasEstimates;
use std::collections::HashMap;
use std::str::Lines;
use std::iter::Peekable;
use std::path::Path;
use std::process::{Command, Stdio};
use std::io::{Result, prelude::*};

pub struct SolcContract {
    pub name: String,
    pub abi: Vec<Abi>,
    pub bin: String,
    pub gas_estimates: GasEstimates,
}

pub enum CompilerInput<'a> {
    Stdin(&'a str),
    Path(&'a Path),
}

macro_rules! next_line {
    ($lines:expr) => {
        $lines.next().expect("Solc changed the output format")
    };
}

macro_rules! assert_line {
    ($lines:expr, $expr:expr) => {
        assert_eq!(next_line!($lines).trim(), $expr, "Solc changed the output format");
    };
}

pub fn compile_contract(input: CompilerInput) -> Result<Vec<SolcContract>> {
    Ok(parse_output(&call_compiler(input, &["--abi", "--gas", "--bin"])?))
}

pub fn parse_output(output: &str) -> Vec<SolcContract> {
    let mut contracts = Vec::new();
    let mut lines = output.lines().peekable();

    // The output sa of solc 0.5.10 is
    //
    // _Blank line_
    // ======= contract_path:ContractName =======
    // *Gas estimation:*
    // construction:
    //    amount + amount = total
    // external:
    //    function_name(args): amount
    // *Binary:*
    // _Output_
    // *Contract JSON ABI*
    // _JSON ABI_
    loop {
        // Each iteration parses a contract

        // Skip blank line
        if lines.next().is_none() {
            // No more contracts to parse
            break;
        }

        let name = parse_name(next_line!(lines));

        assert_line!(lines, "Gas estimation:");
        let gas_estimates = parse_gas_estimates(&mut lines);

        assert_line!(lines, "Binary:");
        let bin = next_line!(lines);

        assert_line!(lines, "Contract JSON ABI");
        let abi = next_line!(lines);

        contracts.push(SolcContract {
            name,
            abi: Abi::from_json_array(abi).expect("Couldn't parse solc JSON abi"),
            bin:bin.to_owned(),
            gas_estimates,
        });
    }

    contracts
}

fn parse_name(line: &str) -> String {
    String::from(line.trim_matches(|c| c == ' ' || c == '=')
        .split(':')
        .last()
        .expect("Solc changed the output format"))
}

fn parse_gas_estimates(lines: &mut Peekable<Lines>) -> GasEstimates {
    assert_line!(lines, "construction:");

    let construction = next_line!(lines)
        .split('=')
        .last()
        .expect("Solc changed the output format")
        .trim();

    let mut external = HashMap::new();
    let mut internal = HashMap::new();

    let mut current = &mut external;

    assert_line!(lines, "external:");

    loop {
        {
            let end_of_estimates = lines.peek()
                .and_then(|&n| if n.trim() == "Binary:" { None } else { Some("") })
                .is_none();
            if end_of_estimates {
                break;
            }
        }

        let line = next_line!(lines).trim();
        if line == "internal:" {
            current = &mut internal;
            continue;
        }

        let mut line = line.split(':');
        let name = line.next()
            .expect("Solc changed the output format")
            .split('(')
            .next()
            .expect("Solc changed the output format");

        let value = line.next()
            .expect("Solc changed the output format")
            .trim();

        current.insert(name.to_owned(), value.to_owned());
    }

    GasEstimates {
        construction: construction.to_owned(),
        external,
        internal
    }
}

pub fn call_compiler(contract: CompilerInput, args: &[&str]) -> Result<String> {
    match contract {
        CompilerInput::Stdin(stdin) => call_compiler_stdin(stdin, args),
        CompilerInput::Path(path) => call_compiler_path(path, args),
    }
}

fn call_compiler_stdin(contract: &str, args: &[&str]) -> Result<String> {
    let mut solc = Command::new("solc");

    for arg in args.iter() {
        solc.arg(arg);
    }

    let mut solc = solc.arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute solc");

    {
        let stdin = solc.stdin.as_mut().expect("Failed to open stdin for solc");
        stdin.write_all(contract.as_bytes()).expect("Failed to write to stdin for solc");
    }

    let output = solc.wait_with_output().expect("Failed to read stdout for solc");
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn call_compiler_path(path: impl AsRef<Path>, args: &[&str]) -> Result<String> {
    let mut solc = Command::new("solc");

    for arg in args.iter() {
        solc.arg(arg);
    }

    let solc = solc.arg(path.as_ref())
        .output()
        .expect("Failed to execute solc");

    if solc.stderr.len() > 0 {
        eprintln!("{}", String::from_utf8_lossy(&solc.stderr));
    }

    Ok(String::from(String::from_utf8_lossy(&solc.stdout)))
}
