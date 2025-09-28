use args::Command as CliCommand;
use svc_core::{AddSectionValues, BuildValues, Command as CoreCommand, DiffValues, InitValues};

mod args;

use clap::Parser;
use svc_core::cli;

fn main() {
    let args = args::SvcArgs::parse();

    let core_command = match args.operation {
        CliCommand::Init(a) => cli::build_init(a.name),
        CliCommand::AddSection(a) => cli::build_add_section(a.name, a.file_name),
        CliCommand::Diff(a) => cli::build_diff(a.hash1, a.hash2),
        CliCommand::Build(a) => cli::build_build(a.name),
    };

    let config = svc_core::Config::new(core_command);

    match config.command {
        svc_core::Command::Init(values) => println!("Init project: {}", values.name),
        svc_core::Command::AddSection(values) => println!("Add section: {}", values.name),
        svc_core::Command::Diff(values) => println!("Diff {} vs {}", values.hash1, values.hash2),
        svc_core::Command::Build(values) => println!("Build report: {}", values.name),
    }
}
