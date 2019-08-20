use crate::abi::Abi;
use serde::{Serialize, Deserialize};
use std::fmt;
use std::collections::HashMap;

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

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Address(pub [u8; 20]);

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
}

impl fmt::Display for Contract {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
