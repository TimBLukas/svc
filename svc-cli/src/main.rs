use clap::{Arg, Command};

mod args;

use args::SvcArgs;
use clap::Parser;

fn main() {
    let args = SvcArgs::parse();
}
