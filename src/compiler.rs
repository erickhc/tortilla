use crate::contract::Contract;
use crate::abi::*;
use std::io::prelude::*;
use std::process::{Command, Stdio};
use std::path::Path;
use std::fs::{File, read_dir};

pub fn compile_str(contract: &str) -> std::io::Result<Vec<Contract>> {
    let abis = compile_abi(contract)?;
    let bins = compile_bin(contract)?;

    Ok(abis
       .into_iter()
       .zip(bins.into_iter())
       // Assume the order of the contracts is always the same from solc
       .map(|((name, abi), (_, bin))| Contract::new(name, abi, bin))
       .collect()
   )
}

pub fn compile_file(file: &mut impl Read) -> std::io::Result<Vec<Contract>> {
    let mut contract = String::new();
    file.read_to_string(&mut contract)?;

    compile_str(&contract)
}

pub fn compile_dir(dir: &Path) -> std::io::Result<Vec<Contract>> {
    let mut contracts = Vec::new();
    for entry in read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let mut file = File::open(path)?;
        contracts.push(compile_file(&mut file)?);
    }
    Ok(contracts.into_iter().flatten().collect())
}

fn compile_abi(contract: &str) -> std::io::Result<Vec<(String, Vec<Abi>)>> {
    let output = call_compiler(contract, &["--abi"])?;
    let mut abis = Vec::with_capacity(2);
    let mut lines = output.lines();
    // The output as of solc 0.5.10 is
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

        abis.push((
            String::from(name),
            Abi::from_json_array(abi).expect("Couldn't parse abi")
        ));
    }

    Ok(abis)
}

fn compile_bin(contract: &str) -> std::io::Result<Vec<(String, String)>> {
    let output = call_compiler(contract, &["--bin"])?;
    let mut bins = Vec::with_capacity(2);
    let mut lines = output.lines();
    // The output as of solc 0.5.10 is
    //
    // _Blank line_
    // ======= path/to/contract/file:ContractName =======
    // Binary:
    // _binary code_
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

        // Skip Binary: line
        lines.next().expect("Solc changed the output format");
        // Get JSON Abi
        let bin = lines.next().expect("Solc changed the output format");

        bins.push((
            String::from(name),
            String::from(bin),
        ));
    }

    Ok(bins)
}

fn call_compiler(contract: &str, args: &[&str]) -> std::io::Result<String> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{Write, SeekFrom};

    fn cmp_migrations_contract(contract: &Contract) {
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

        let output_bin = "608060405234801561001057600080fd5b50336000806101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055506102b7806100606000396000f3fe608060405234801561001057600080fd5b506004361061004c5760003560e01c80630900f01014610051578063445df0ac146100955780638da5cb5b146100b3578063fdacd576146100fd575b600080fd5b6100936004803603602081101561006757600080fd5b81019080803573ffffffffffffffffffffffffffffffffffffffff16906020019092919050505061012b565b005b61009d6101f7565b6040518082815260200191505060405180910390f35b6100bb6101fd565b604051808273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200191505060405180910390f35b6101296004803603602081101561011357600080fd5b8101908080359060200190929190505050610222565b005b6000809054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614156101f45760008190508073ffffffffffffffffffffffffffffffffffffffff1663fdacd5766001546040518263ffffffff1660e01b815260040180828152602001915050600060405180830381600087803b1580156101da57600080fd5b505af11580156101ee573d6000803e3d6000fd5b50505050505b50565b60015481565b6000809054906101000a900473ffffffffffffffffffffffffffffffffffffffff1681565b6000809054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673";

        assert!(contract.bin.starts_with(output_bin));
    }

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
        "#.trim();

        let contracts = compile_str(input).expect("Error compiling contract");
        assert_eq!(contracts.len(), 1);
        let contract = &contracts[0];

        cmp_migrations_contract(contract);
    }

    #[test]
    fn test_compile_from_file() {
        let mut tmpfile: File = tempfile::tempfile().unwrap();
        write!(tmpfile, "{}", r#"
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
            }"#.trim()
        ).unwrap();

        tmpfile.seek(SeekFrom::Start(0)).unwrap();
        let contracts = compile_file(&mut tmpfile)
            .expect("Couldn't compile contract from file");
        assert_eq!(contracts.len(), 1);
        let contract = &contracts[0];
        cmp_migrations_contract(contract);
    }

    #[test]
    fn test_compile_from_dir() {
        let dir = tempfile::tempdir().unwrap();
        let mut tmpfile: File = File::create(dir.path().join("Migrations.sol")).unwrap();
        write!(tmpfile, "{}", r#"
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
            }"#.trim()
        ).unwrap();

        let contracts = compile_dir(dir.path())
            .expect("Couldn't compile contract from file");
        assert_eq!(contracts.len(), 1);
        let contract = &contracts[0];
        cmp_migrations_contract(contract);
    }
}
