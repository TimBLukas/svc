use args::Command as CliCommand;

mod args;

use clap::Parser;
use svc_core::cli;

fn main() {
    let args = args::SvcArgs::parse();

    let core_command = match args.operation {
        CliCommand::Init(a) => cli::build_init(a.name),
        CliCommand::AddSection(a) => cli::build_add_section(a.name),
        CliCommand::Diff(a) => cli::build_diff(a.hash1, a.hash2),
        CliCommand::Build(a) => cli::build_build(a.name),
    };

    let config = svc_core::Config::new(core_command);

    svc_core::handle_command(config);
}
