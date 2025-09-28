/// All possible Commands summarized

#[derive(Debug)]
pub enum Command {
    Init(InitValues),
    AddSection(AddSectionValues),
    Diff(DiffValues),
    Build(BuildValues),
}

/// Structs for the diffrent commands
#[derive(Debug)]
pub struct InitValues {
    pub name: String,
}

#[derive(Debug)]
pub struct AddSectionValues {
    pub name: String,
    pub file_name: String,
}

#[derive(Debug)]
pub struct DiffValues {
    pub hash1: String,
    pub hash2: String,
}

#[derive(Debug)]
pub struct BuildValues {
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
