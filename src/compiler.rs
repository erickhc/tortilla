use crate::contract::Contract;
use crate::abi::*;
use std::io::prelude::*;
use std::process::{Command, Stdio};
use std::str::FromStr;

pub fn compile(contract: &str) -> std::io::Result<Vec<Contract>> {
    let mut solc = Command::new("solc")
        .arg("--abi")
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute solc");

    {
        let stdin = solc.stdin.as_mut().expect("Failed to open stdin for solc");
        stdin.write_all(contract.as_bytes()).expect("Failed to write to stdin for solc");
    }

    let output = solc.wait_with_output().expect("Failed to read stdout for solc");
    let output = String::from_utf8_lossy(&output.stdout);

    let mut contracts = Vec::with_capacity(2);
    let mut lines = output.lines();
    // The output of solc 0.5.10 is
    //
    // _Blank line_
    // ======= path/to/contract/file:ContractName =======
    // Contract JSON ABI
    // _json abi_
    loop {
        // Skip blank line
        if lines.next().is_none() {
            break;
        }

        // Get ContractName
        let name = lines.next().expect("Solc changed the output format")
            .trim_matches(|c| c == ' ' || c == '=')
            .split(':')
            .last()
            .expect("Solc changed the output format");

        // Skip Contract JSON ABI line
        lines.next().expect("Solc changed the output format");
        // Get JSON Abi
        let abi = lines.next().expect("Solc changed the output format");

        contracts.push(Contract::new(
            String::from(name),
            Abi::from_json_array(abi).expect("Couldn't parse abi")
        ));
    }

    Ok(contracts)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compile_simple_contract() {
        let input = r#"
            pragma solidity ^0.5.0;

            contract Migrations {
              address public owner;
              uint public last_completed_migration;

              modifier restricted() {
                if (msg.sender == owner) _;
              }

              constructor() public {
                owner = msg.sender;
              }

              function setCompleted(uint completed) public restricted {
                last_completed_migration = completed;
              }

              function upgrade(address new_address) public restricted {
                Migrations upgraded = Migrations(new_address);
                upgraded.setCompleted(last_completed_migration);
              }
            }
        "#;

        let contracts = compile(input).expect("Error compiling contract");
        assert_eq!(contracts.len(), 1);
        let contract = &contracts[0];
        assert_eq!(contract.name, "Migrations");

        let abi = vec![
            Abi::Function(Function {
                constant: false,
                inputs: vec![
                    Variable {
                        name: "new_address".to_owned(),
                        r#type: "address".to_owned(),
                    },
                ],
                name: "upgrade".to_owned(),
                outputs: vec![],
                payable: false,
                stateMutability: "nonpayable".to_owned(),
                r#type: "function".to_owned(),
            }),
            Abi::Function(Function {
                constant: true,
                inputs: vec![],
                name: "last_completed_migration".to_owned(),
                outputs: vec![
                    Variable {
                        name: "".to_owned(),
                        r#type: "uint256".to_owned(),
                    },
                ],
                payable: false,
                stateMutability: "view".to_owned(),
                r#type: "function".to_owned(),
            }),
            Abi::Function(Function {
                constant: true,
                inputs: vec![],
                name: "owner".to_owned(),
                outputs: vec![
                    Variable {
                        name: "".to_owned(),
                        r#type: "address".to_owned(),
                    },
                ],
                payable: false,
                stateMutability: "view".to_owned(),
                r#type: "function".to_owned(),
            }),
            Abi::Function(Function {
                constant: false,
                inputs: vec![
                    Variable {
                        name: "completed".to_owned(),
                        r#type: "uint256".to_owned(),
                    },
                ],
                name: "setCompleted".to_owned(),
                outputs: vec![ ],
                payable: false,
                stateMutability: "nonpayable".to_owned(),
                r#type: "function".to_owned(),
            }),
            Abi::Constructor(Constructor {
                inputs: vec![ ],
                payable: false,
                stateMutability: "nonpayable".to_owned(),
                r#type: "constructor".to_owned(),
            }),
        ];

        assert_eq!(contract.abi, abi);
    }
}
