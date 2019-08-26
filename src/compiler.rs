use crate::contract::Contract;
use crate::solc::*;
use std::io::Result;
use std::path::Path;
use std::fs::{read_dir};

macro_rules! solc_to_contracts {
    ($expr:expr) => {
        $expr.into_iter()
            .map(|c| Contract::from_solc_contract(c))
            .collect();
    }
}

pub fn compile_str(contract: &str) -> Result<Vec<Contract>> {
    let contracts = compile_contract(CompilerInput::Stdin(contract))?;

    Ok(solc_to_contracts!(contracts))
}

pub fn compile_file(file: impl AsRef<Path>) -> Result<Vec<Contract>> {
    let contracts = compile_contract(CompilerInput::Path(file.as_ref()))?;

    Ok(solc_to_contracts!(contracts))
}

pub fn compile_dir(dir: impl AsRef<Path>) -> Result<Vec<Contract>> {
    let mut contracts = Vec::new();
    for entry in read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        contracts.push(compile_file(path)?);
    }
    Ok(contracts.into_iter().flatten().collect())
}

pub fn compile_path(path: impl AsRef<Path>) -> Result<Vec<Contract>> {
    if path.as_ref().is_file() {
        compile_file(path)
    } else {
        compile_dir(path)
    }
}

pub fn compile_paths(paths: &[impl AsRef<Path>]) -> Result<Vec<Contract>> {
    paths.into_iter()
        .map(|p| compile_path(p))
        .collect::<Result<Vec<Vec<Contract>>>>()
        .and_then(|c| Ok(c.into_iter().flatten().collect()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::abi::*;
    use std::fs::File;
    use std::io::Write;

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
        let input = include_str!("../tests/contracts/Migrations.sol");

        let contracts = compile_str(input).expect("Error compiling contract");
        assert_eq!(contracts.len(), 1);
        let contract = &contracts[0];

        cmp_migrations_contract(contract);
    }

    #[test]
    fn test_compile_from_file() {
        let contracts = compile_file("tests/contracts/Migrations.sol")
            .expect("Couldn't compile contract from file");
        assert_eq!(contracts.len(), 1);
        let contract = &contracts[0];
        cmp_migrations_contract(contract);
    }

    #[test]
    fn test_compile_from_dir() {
        let dir = tempfile::tempdir().unwrap();
        let mut tmpfile: File = File::create(dir.path().join("Migrations.sol")).unwrap();
        write!(tmpfile, "{}", include_str!("../tests/contracts/Migrations.sol")).unwrap();

        let contracts = compile_dir(dir.path())
            .expect("Couldn't compile contract from file");
        assert_eq!(contracts.len(), 1);
        let contract = &contracts[0];
        cmp_migrations_contract(contract);
    }
}
