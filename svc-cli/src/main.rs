use args::Command as CliCommand;
use svc_core::{AddSectionValues, BuildValues, Command as CoreCommand, DiffValues, InitValues};

mod args;

use clap::Parser;

fn main() {
    let args = args::SvcArgs::parse();

    let core_command = map_cli_to_core(args.operation);
    let config = svc_core::Config::new(core_command);

    match config.command {
        svc_core::Command::Init(values) => println!("Init project: {}", values.name),
        svc_core::Command::AddSection(values) => println!("Add section: {}", values.name),
        svc_core::Command::Diff(values) => println!("Diff {} vs {}", values.hash1, values.hash2),
        svc_core::Command::Build(values) => println!("Build report: {}", values.name),
    }
}

fn map_cli_to_core(command: CliCommand) -> CoreCommand {
    match command {
        CliCommand::Init(a) => CoreCommand::Init(InitValues { name: a.name }),
        CliCommand::AddSection(a) => CoreCommand::AddSection(AddSectionValues {
            name: a.name,
            file_name: a.file_name,
        }),
        CliCommand::Diff(a) => CoreCommand::Diff(DiffValues {
            hash1: a.hash1,
            hash2: a.hash2,
        }),
        CliCommand::Build(a) => CoreCommand::Build(BuildValues { name: a.name }),
    }
}
