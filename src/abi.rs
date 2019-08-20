use serde::{Serialize, Deserialize};
use std::str::FromStr;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Abi {
    Function(Function),
    Constructor(Constructor),
    Event(Event),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Function {
    pub constant: bool,
    pub inputs: Vec<Variable>,
    pub name: String,
    pub outputs: Vec<Variable>,
    pub payable: bool,
    pub stateMutability: String,
    pub r#type: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Constructor {
    pub inputs: Vec<Variable>,
    pub payable: bool,
    pub stateMutability: String,
    pub r#type: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Event {
    pub anonymous: bool,
    pub inputs: Vec<EventVariable>,
    pub name: String,
    r#type: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Variable {
    pub name: String,
    pub r#type: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EventVariable {
    pub indexed: bool,
    pub name: String,
    pub r#type: String,
}

impl Abi {
    pub fn from_json_array(s: &str) -> Result<Vec<Self>, serde_json::error::Error> {
        serde_json::from_str(s)
    }
}

impl FromStr for Abi {
    type Err = serde_json::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

#[cfg(test)]
mod tests {
    use super::Abi;
    use std::str::FromStr;

    #[test]
    fn test_function_one_input_no_output_from_str() {
        let input = r#"
            {
              "constant": false,
              "inputs": [
                {
                  "name": "new_address",
                  "type": "address"
                }
              ],
              "name": "upgrade",
              "outputs": [],
              "payable": false,
              "stateMutability": "nonpayable",
              "type": "function"
            }"#;

        let abi = Abi::from_str(&input)
            .expect("Couldn't parse the input");

        match abi {
            Abi::Function(func) => {
                assert_eq!(func.constant, false);
                assert_eq!(func.inputs.len(), 1);
                assert_eq!(func.inputs[0].name, "new_address");
                assert_eq!(func.inputs[0].r#type, "address");
                assert_eq!(func.name, "upgrade");
                assert_eq!(func.outputs.len(), 0);
                assert_eq!(func.payable, false);
                assert_eq!(func.stateMutability, "nonpayable");
                assert_eq!(func.r#type, "function");
            },
            Abi::Constructor(_) => { unreachable!(); },
            Abi::Event(_) => { unreachable!() },
        }
    }

    #[test]
    fn test_public_attribute() {
        let input = r#"
            {
              "constant": true,
              "inputs": [],
              "name": "last_completed_migration",
              "outputs": [
                {
                  "name": "",
                  "type": "uint256"
                }
              ],
              "payable": false,
              "stateMutability": "view",
              "type": "function"
            }"#;

        let abi = Abi::from_str(&input)
            .expect("Couldn't parse the input");

        match abi {
            Abi::Function(func) => {
                assert_eq!(func.constant, true);
                assert_eq!(func.inputs.len(), 0);
                assert_eq!(func.name, "last_completed_migration");
                assert_eq!(func.outputs.len(), 1);
                assert_eq!(func.outputs[0].name, "");
                assert_eq!(func.outputs[0].r#type, "uint256");
                assert_eq!(func.payable, false);
                assert_eq!(func.stateMutability, "view");
                assert_eq!(func.r#type, "function");
            },
            Abi::Constructor(_) => { unreachable!(); },
            Abi::Event(_) => { unreachable!() },
        }
    }

    #[test]
    fn test_constructor_no_inputs_from_str() {
        let input = r#"
            {
              "inputs": [],
              "payable": false,
              "stateMutability": "nonpayable",
              "type": "constructor"
            }"#;

        let abi = Abi::from_str(&input)
            .expect("Couldn't parse the input");

        match abi {
            Abi::Function(_) => { unreachable!(); },
            Abi::Constructor(constructor) => {
                assert_eq!(constructor.inputs.len(), 0);
                assert_eq!(constructor.payable, false);
                assert_eq!(constructor.stateMutability, "nonpayable");
                assert_eq!(constructor.r#type, "constructor");
            },
            Abi::Event(_) => { unreachable!() },
        }
    }

    #[test]
    fn test_constructor_one_input_from_str() {
        let input = r#"
            {
              "inputs": [
                {
                  "name": "proposalNames",
                  "type": "bytes32[]"
                }
              ],
              "payable": false,
              "stateMutability": "nonpayable",
              "type": "constructor"
            } "#;

        let abi = Abi::from_str(&input)
            .expect("Couldn't parse the input");

        match abi {
            Abi::Function(_) => { unreachable!(); },
            Abi::Constructor(constructor) => {
                assert_eq!(constructor.inputs.len(), 1);
                assert_eq!(constructor.inputs[0].name, "proposalNames");
                assert_eq!(constructor.inputs[0].r#type, "bytes32[]");
                assert_eq!(constructor.payable, false);
                assert_eq!(constructor.stateMutability, "nonpayable");
                assert_eq!(constructor.r#type, "constructor");
            },
            Abi::Event(_) => { unreachable!() },
        }
    }

    #[test]
    fn test_event() {
        let input = r#"
            {
              "anonymous": false,
              "inputs": [
                {
                  "indexed": false,
                  "name": "winner",
                  "type": "address"
                },
                {
                  "indexed": false,
                  "name": "amount",
                  "type": "uint256"
                }
              ],
              "name": "AuctionEnded",
              "type": "event"
            } "#;

        let abi = Abi::from_str(&input)
            .expect("Couldn't parse the input");

        match abi {
            Abi::Function(_) => { unreachable!(); },
            Abi::Constructor(_) => { unreachable!(); },
            Abi::Event(event) => {
                assert_eq!(event.anonymous, false);
                assert_eq!(event.inputs.len(), 2);
                assert_eq!(event.inputs[0].indexed, false);
                assert_eq!(event.inputs[0].name, "winner");
                assert_eq!(event.inputs[0].r#type, "address");
                assert_eq!(event.inputs[1].indexed, false);
                assert_eq!(event.inputs[1].name, "amount");
                assert_eq!(event.inputs[1].r#type, "uint256");
                assert_eq!(event.name, "AuctionEnded");
                assert_eq!(event.r#type, "event");
            },
        }
    }

    #[test]
    fn test_array_event() {
        let input = r#"[
            {
              "anonymous": false,
              "inputs": [
                {
                  "indexed": false,
                  "name": "winner",
                  "type": "address"
                },
                {
                  "indexed": false,
                  "name": "amount",
                  "type": "uint256"
                }
              ],
              "name": "AuctionEnded",
              "type": "event"
            }]"#;

        let abis = Abi::from_json_array(&input)
            .expect("Couldn't parse the input");

        assert_eq!(abis.len(), 1);
        let abi = &abis[0];

        match abi {
            Abi::Function(_) => { unreachable!(); },
            Abi::Constructor(_) => { unreachable!(); },
            Abi::Event(event) => {
                assert_eq!(event.anonymous, false);
                assert_eq!(event.inputs.len(), 2);
                assert_eq!(event.inputs[0].indexed, false);
                assert_eq!(event.inputs[0].name, "winner");
                assert_eq!(event.inputs[0].r#type, "address");
                assert_eq!(event.inputs[1].indexed, false);
                assert_eq!(event.inputs[1].name, "amount");
                assert_eq!(event.inputs[1].r#type, "uint256");
                assert_eq!(event.name, "AuctionEnded");
                assert_eq!(event.r#type, "event");
            },
        }
    }
}
