# tortilla
tortilla is a wrapper around solc, running it and parsing its output.

# How to build

Prerequisites:
* Have the official solidity compiler installed, you can follow the instructions (here)[https://solidity.readthedocs.io/en/v0.6.2/installing-solidity.html].
  * Note: It has only been tested with version `0.5.11`.

You can test if you have it installed by running:

```bash
$ solc --version
```

You should see something like this:

```
solc, the solidity compiler commandline interface
Version: 0.5.11+commit.22be8592.Linux.g++
```

Clone the repository:

```bash
$ git clone https://github.com/erickhc/tortilla.git
$ cd tortilla
$ cargo test
$ cargo install --path .
```

Create the file `HelloWorld.sol` with the following content:

```solidity
pragma solidity ^0.6.0;

contract HelloWorld {
  function helloWorld() external pure returns (string memory) {
    return "Hello, World!";
  }
}
```

Run `tortilla` on the file:

```bash
$ # `-o .` Specifies the output for the compiler, where you want the .json file to be created
$ tortilla HelloWorld.sol -o .
```

You should see the following file `HelloWorld.json`:

```json
{"name":"HelloWorld","abi":[{"type":"function","name":"helloWorld","inputs":[],"outputs":[{"name":"","type":"string","components":null}],"stateMutability":"pure"}],"bin":"608060405234801561001057600080fd5b5061011e806100206000396000f3fe6080604052348015600f57600080fd5b506004361060285760003560e01c8063c605f76c14602d575b600080fd5b603360ab565b6040518080602001828103825283818151815260200191508051906020019080838360005b8381101560715780820151818401526020810190506058565b50505050905090810190601f168015609d5780820380516001836020036101000a031916815260200191505b509250505060405180910390f35b60606040518060400160405280600d81526020017f48656c6c6f2c20576f726c64210000000000000000000000000000000000000081525090509056fea2646970667358221220ec52c46cd904fdc3f6ffdb72721846239a5bb061487afb8d1ba689f6b12b664564736f6c63430006020033","gas_estimates":{"construction":"57305","external":{"helloWorld":"infinite"},"internal":{}},"networks":{}}
```
