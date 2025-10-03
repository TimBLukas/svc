// Diff Command Value Structure
#[derive(Debug)]
pub struct DiffCommand {
    pub hash1: String,
    pub hash2: String,
}

impl DiffCommand {
    pub fn new(hash1: String, hash2: String) -> Self {
        DiffCommand { hash1, hash2 }
    }
    pub fn execute_diff_command(&self) {}
}
