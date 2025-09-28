use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct SvcArgs {
    #[clap(subcommand)]
    pub operation: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Initialize an empty svc project
    Init(InitArgs),
    /// Create a new section within an existing svc project
    AddSection(AddSectionArgs),
    /// Find differences between two versions using their hashes
    Diff(DiffArgs),
    /// Merge all sections into a single report
    Build(BuildArgs),
}

#[derive(Debug, Args)]
pub struct InitArgs {
    /// Name of the project
    pub name: String,
}

#[derive(Debug, Args)]
pub struct AddSectionArgs {
    /// Name of the section
    pub name: String,
    /// Filename of the section
    pub file_name: String,
}

#[derive(Debug, Args)]
pub struct DiffArgs {
    /// Hash value of the first commit
    pub hash1: String,
    /// Hash value of the second commit
    pub hash2: String,
}

#[derive(Debug, Args)]
pub struct BuildArgs {
    /// Name of the output file
    pub name: String,
}
