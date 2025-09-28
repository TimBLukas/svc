use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct SvcArgs {
    #[clap(subcommand)]
    pub operation: Operation,
}

#[derice(Debug, Subcommand)]
pub enum Operation {
    /// Initialize an Empty svc Project
    Init(InitCommand),
    /// Create a new Section within an existing svc project
    AddSection(AddSectionCommand),
    /// Find differences between to seperate Versions using their hashes
    Diff(DiffCommand),
    /// Merge all sections into a single report
    Build(BuildCommand),
}
