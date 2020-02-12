use std::path::{Path, PathBuf};

pub struct Config {
    pub inputs: Vec<PathBuf>,
    pub watch: bool,
    pub output: String,
    pub pretty_print: bool,
    pub gas: bool,
}

impl Config {
    pub fn new(inputs: &[impl AsRef<Path>]) -> Self {
        Self {
            inputs: inputs.iter().map(|i| i.as_ref().to_owned()).collect(),
            watch: false,
            output: String::new(),
            pretty_print: false,
            gas: false,
        }
    }

    pub fn watch(mut self, watch: bool) -> Self {
        self.watch = watch;
        self
    }

    pub fn output(mut self, output: &str) -> Self {
        self.output = output.to_owned();
        self
    }

    pub fn pretty_print(mut self, pretty_print: bool) -> Self {
        self.pretty_print = pretty_print;
        self
    }

    pub fn gas(mut self, gas: bool) -> Self {
        self.gas = gas;
        self
    }
}
