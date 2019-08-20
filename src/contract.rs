use crate::abi::Abi;

pub struct Contract {
    pub name: String,
    pub abi: Vec<Abi>,
}

impl Contract {
    pub fn new(name: String, abi: Vec<Abi>) -> Self {
        Self {
            name,
            abi,
        }
    }
}
