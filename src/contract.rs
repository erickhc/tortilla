use crate::abi::{Abi, Function};
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
    pub networks: HashMap<String, Network>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Network {
    address: Address,
}

impl Network {
    pub fn new(address: Address) -> Self {
        Self {
            address,
        }
    }
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

    pub fn write_to_dir(&self, dir: impl AsRef<Path>) -> io::Result<PathBuf> {
        let dir = dir.as_ref();
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

    pub fn write_to_dir_pretty_print(&self, dir: impl AsRef<Path>) -> io::Result<PathBuf> {
        let dir = dir.as_ref();
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

    pub fn get_methods(&self) -> HashMap<String, Function> {
        let mut methods = HashMap::new();
        for el in self.abi.iter() {
            if let Abi::Function(f) = el {
                methods.insert(f.name.clone(), f.clone());
            }
        }
        methods
    }

    pub fn add_network(&mut self, net_version: &str, address: Address) {
        self.networks.insert(net_version.to_owned(), Network::new(address));
    }

    pub fn get_address(&self, net_version: &str) -> Option<Address> {
        self.networks.get(net_version).and_then(|n| Some(n.address))
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

    #[test]
    fn test_get_methods() {
        let contracts = compiler::compile_file("tests/contracts/Migrations.sol").unwrap();
        let contract = &contracts[0];

        let methods = contract.get_methods();
        assert_eq!(methods.keys().len(), 4);

        assert_eq!(methods.contains_key("owner"), true);
        assert_eq!(methods.contains_key("last_completed_migration"), true);
        assert_eq!(methods.contains_key("setCompleted"), true);
        assert_eq!(methods.contains_key("upgrade"), true);
    }

    #[test]
    fn test_add_network() {
        let mut contracts = compiler::compile_file("tests/contracts/Migrations.sol").unwrap();
        let mut contract = contracts.remove(0);

        contract.add_network("1566487350707", "e78a0f7e598cc8b0bb87894b0f60dd2a88d6a8ab".parse().unwrap());

        assert_eq!(
            contract.get_address("1566487350707"),
            Some("e78a0f7e598cc8b0bb87894b0f60dd2a88d6a8ab".parse().unwrap())
        );
    }
}
