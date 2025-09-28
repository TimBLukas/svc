use crate::commands::Command;

pub struct Config {
    pub command: Command,
}

impl Config {
    pub fn new(command: Command) -> Self {
        Config { command }
    }
}
