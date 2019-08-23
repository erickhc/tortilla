use std::path::{Path, PathBuf};

pub struct Config {
    pub inputs: Vec<PathBuf>,
    pub watch: bool,
    pub output: String,
}

impl Config {
    pub fn new(inputs: &Vec<impl AsRef<Path>>) -> Self {
        Self {
            inputs: inputs.into_iter().map(|i| i.as_ref().to_owned()).collect(),
            watch: false,
            output: String::new(),
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
}
