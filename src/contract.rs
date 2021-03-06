use crate::abi::{Abi, Function};
use crate::solc::SolcContract;
use serde::{Serialize, Deserialize};
use std::fmt;
use std::fs::{File, DirBuilder};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use ethereum_types::H160;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Contract {
    pub name: String,
    pub abi: Vec<Abi>,
    pub bin: String,
    pub gas_estimates: Option<GasEstimates>,
    pub networks: HashMap<String, Network>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct GasEstimates {
    pub construction: String,
    pub external: HashMap<String, String>,
    pub internal: HashMap<String, String>,
}

impl Contract {
    pub fn new(name: String, abi: Vec<Abi>, bin: String) -> Self {
        Self {
            name,
            abi,
            bin,
            networks: HashMap::new(),
            gas_estimates: None,
        }
    }

    pub fn pretty_print(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }

    pub fn write_to_dir(&self, dir: impl AsRef<Path>, pretty_print: bool) -> io::Result<PathBuf> {
        let dir = dir.as_ref();
        if !dir.exists() {
            DirBuilder::new()
                .create(dir)?;
        }

        let mut output_file = PathBuf::from(dir);
        output_file.push(&self.name);
        output_file.set_extension("json");

        let mut file = File::create(&output_file)?;
        if pretty_print {
            write!(file, "{}", self.pretty_print())?;
        } else {
            write!(file, "{}", self)?;
        }

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
        self.networks.get(net_version).map(|n| n.address)
    }

    pub fn from_solc_contract(c: SolcContract) -> Self {
        Self {
            name: c.name,
            abi: c.abi,
            bin: c.bin,
            gas_estimates: Some(c.gas_estimates),
            networks: HashMap::new(),
        }
    }

    pub fn gas_estimates_to_string(&self) -> String {
        let mut output = Vec::new();

        if let Some(gas) = &self.gas_estimates {
            let biggest = gas.external.keys()
                .chain(gas.internal.keys())
                .fold("construction".len(), |acc, x| std::cmp::max(acc, x.len()));

            macro_rules! pad {
                ($expr:expr) => {
                    std::iter::repeat(' ').take(biggest - $expr.len()).collect::<String>();
                };
            }

            output.push(format!("{}construction: {}", pad!("construction"), gas.construction));

            if gas.external.keys().len() > 0 {
                output.push(format!("{}external:", pad!("external")));
                let mut sorted_keys = gas.external.keys().collect::<Vec<&String>>();
                sorted_keys.sort();

                for &name in sorted_keys.iter() {
                    output.push(format!("{}{}: {}", pad!(name), name, gas.external.get(name).unwrap()));
                }
            }

            if gas.internal.keys().len() > 0 {
                output.push(format!("{}internal:", pad!("internal")));
                let mut sorted_keys = gas.internal.keys().collect::<Vec<&String>>();
                sorted_keys.sort();

                for &name in sorted_keys.iter() {
                    output.push(format!("{}{}: {}", pad!(name), name, gas.internal.get(name).unwrap()));
                }
            }
        }

        output.join("\n")
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::error::Error> {
        serde_json::from_str(json)
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

        let mut output_file = File::open(contract.write_to_dir(path, false).unwrap()).unwrap();
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

    #[test]
    fn test_from_json() {
        let mut contracts = compiler::compile_file("tests/contracts/Migrations.sol").unwrap();
        let contract = contracts.remove(0);

        let json = contract.pretty_print();

        let from_json = Contract::from_json(&json).expect("Couldn't parse contract");

        assert_eq!(contract, from_json);
    }
}
