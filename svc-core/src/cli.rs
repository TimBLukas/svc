use crate::commands::{AddSectionCommand, BuildCommand, Commands, DiffCommand, InitCommand};

pub fn build_init(project_name: String) -> Commands {
    Commands::Init(InitCommand::new(project_name))
}

pub fn build_add_section(section_name: String) -> Commands {
    Commands::AddSection(AddSectionCommand::new(section_name))
}

pub fn build_diff(hash1: String, hash2: String) -> Commands {
    Commands::Diff(DiffCommand::new(hash1, hash2))
}

pub fn build_build(name: String) -> Commands {
    Commands::Build(BuildCommand::new(name))
}
