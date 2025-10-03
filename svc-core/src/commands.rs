// Sub Modules
pub mod add_section;
pub mod build;
pub mod commit;
pub mod diff;
pub mod init;
pub mod status;

// Enum containg all possible commands
#[derive(Debug)]
pub enum Commands {
    Init(init::InitCommand),
    AddSection(add_section::AddSectionCommand),
    Diff(diff::DiffCommand),
    Build(build::BuildCommand),
    Commit(commit::CommitCommand),
    Status,
}

// Re-Exports
pub use add_section::AddSectionCommand;
pub use build::BuildCommand;
pub use commit::CommitCommand;
pub use diff::DiffCommand;
pub use init::InitCommand;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
