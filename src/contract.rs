use crate::abi::Abi;
use serde::{Serialize, Deserialize};
use std::fmt;
use std::fs::{File, DirBuilder};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use ethereum_types::H160;

#[derive(Debug, Serialize, Deserialize)]
pub struct Contract {
    pub name: String,
    pub abi: Vec<Abi>,
    pub bin: String,
    pub networks: HashMap<Address, Network>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Network {
    address: Address,
}

pub type Address = H160;

impl Contract {
    pub fn new(name: String, abi: Vec<Abi>, bin: String) -> Self {
        Self {
            name,
            abi,
            bin,
            networks: HashMap::new(),
        }
    }

    pub fn pretty_print(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }

    pub fn write_to_dir(&self, dir: &Path) -> io::Result<PathBuf> {
        if !dir.exists() {
            DirBuilder::new()
                .create(dir)?;
        }

        let mut output_file = PathBuf::from(dir);
        output_file.push(&self.name);
        output_file.set_extension("json");

        let mut file = File::create(&output_file)?;
        write!(file, "{}", self)?;

        Ok(output_file)
    }

    pub fn write_to_dir_pretty_print(&self, dir: &Path) -> io::Result<PathBuf> {
        if !dir.exists() {
            DirBuilder::new()
                .create(dir)?;
        }

        let mut output_file = PathBuf::from(dir);
        output_file.push(&self.name);
        output_file.set_extension("json");

        let mut file = File::create(&output_file)?;
        write!(file, "{}", self.pretty_print())?;

        Ok(output_file)
    }
}

impl fmt::Display for Contract {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler;
    use std::io::Read;

    #[test]
    fn test_to_dir_pretty_print() {
        let contracts = compiler::compile_file("tests/contracts/Migrations.sol").unwrap();
        let contract = &contracts[0];
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path();

        let mut output_file = File::open(contract.write_to_dir_pretty_print(path).unwrap()).unwrap();
        let mut output_content = String::new();
        output_file.read_to_string(&mut output_content).unwrap();

        assert_eq!(output_content, contract.pretty_print());
    }

    #[test]
    fn test_to_dir() {
        let contracts = compiler::compile_file("tests/contracts/Migrations.sol").unwrap();
        let contract = &contracts[0];
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path();

        let mut output_file = File::open(contract.write_to_dir(path).unwrap()).unwrap();
        let mut output_content = String::new();
        output_file.read_to_string(&mut output_content).unwrap();

        assert_eq!(output_content, contract.to_string());
    }
}
