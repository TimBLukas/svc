use crate::{AddSectionValues, BuildValues, Command, DiffValues, InitValues};

pub fn build_init(name: String) -> Command {
    Command::Init(InitValues { name })
}

pub fn build_add_section(name: String) -> Command {
    Command::AddSection(AddSectionValues { name })
}

pub fn build_diff(hash1: String, hash2: String) -> Command {
    Command::Diff(DiffValues { hash1, hash2 })
}

pub fn build_build(name: String) -> Command {
    Command::Build(BuildValues { name })
}
