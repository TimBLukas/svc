use crate::commands::Commands;

pub struct Config {
    pub command: Commands,
}

impl Config {
    pub fn new(command: Commands) -> Self {
        Config { command }
    }
}
